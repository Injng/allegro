use crate::insert;
use crate::models::{Admin, User};
use crate::Response;

use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Deserialize, Serialize};

/// A request to add an artist, with type either performer, composer, or songwriter
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddArtistRequest {
    pub name: String,
    pub description: Option<String>,
    pub has_image: bool,
    pub artist_type: String,
    pub token: String,
}

/// A request to add a release
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddReleaseRequest {
    pub name: String,
    pub performer_ids: Vec<i32>,
    pub description: Option<String>,
    pub has_image: bool,
    pub token: String,
}

/// A request to add a piece
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddPieceRequest {
    pub name: String,
    pub movements: Option<i32>,
    pub composer_ids: Vec<i32>,
    pub songwriter_ids: Option<Vec<i32>>,
    pub description: Option<String>,
    pub token: String,
}

/// A request to add a recording
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddRecordingRequest {
    pub piece_id: i32,
    pub release_id: i32,
    pub performer_ids: Vec<i32>,
    pub track_number: i32,
    pub token: String,
}

/// Given a token, return whether the user is an admin
fn check_admin(
    token: String,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> bool {
    use crate::schema::admin;
    use crate::schema::users;

    // check if the token is valid
    let user_res = users::dsl::users
        .filter(users::dsl::session.eq(Some(token)))
        .first::<User>(conn);
    let user: User = match user_res {
        Ok(_) => user_res.unwrap(),
        Err(_) => {
            return false;
        }
    };

    // check if the user is an admin
    let admin_user_res = admin::dsl::admin
        .filter(admin::dsl::username.eq(&user.username))
        .first::<Admin>(conn);
    match admin_user_res {
        Ok(_) => (),
        Err(_) => {
            return false;
        }
    };

    return true;
}

/// Add a recording to the database, returning the file path of the new recording
fn db_addrecording<T>(
    addrecording_req: AddRecordingRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::{pieces, recording_performers, recordings};

    // check for admin privileges
    let is_admin: bool = check_admin(addrecording_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: "User is not an admin".to_string(),
        };
    }

    // get the piece name from the id
    let piece_name_res = pieces::dsl::pieces
        .filter(pieces::dsl::id.eq(addrecording_req.piece_id))
        .select(pieces::dsl::name)
        .first::<String>(conn);
    let piece_name: String = match piece_name_res {
        Ok(_) => piece_name_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: "Piece id incorrect or piece not found".to_string(),
            };
        }
    };

    // insert the new recording into the database
    let new_recording = insert::NewRecording {
        piece_name,
        piece_id: addrecording_req.piece_id,
        release_id: addrecording_req.release_id,
        track_number: addrecording_req.track_number,
        file_path: None, // initially set to None, will update after getting ID
    };

    // insert recording and get its ID
    let recording_id: i32 = diesel::insert_into(recordings::dsl::recordings)
        .values(&new_recording)
        .returning(recordings::dsl::id)
        .get_result(conn)
        .unwrap();

    // generate the file path using the recording ID
    let new_file_path = format!("recording-{}", recording_id);

    // update the recording with the generated file path
    diesel::update(recordings::dsl::recordings.find(recording_id))
        .set(recordings::dsl::file_path.eq(&new_file_path))
        .execute(conn)
        .unwrap();

    // insert performer relationships
    for performer_id in addrecording_req.performer_ids {
        let _ = diesel::insert_into(recording_performers::table)
            .values((
                recording_performers::recording_id.eq(recording_id),
                recording_performers::performer_id.eq(performer_id),
            ))
            .execute(conn);
    }

    Response {
        success: true,
        message: new_file_path,
    }
}

/// Add a piece to the database
fn db_addpiece<T>(
    addpiece_req: AddPieceRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::{piece_composers, piece_songwriters, pieces};

    // check for admin privileges
    let is_admin: bool = check_admin(addpiece_req.token, conn);
    if !is_admin {
        return Response {
            success: false,
            message: "User is not an admin".to_string(),
        };
    }

    // insert the new piece into the database
    let new_piece = insert::NewPiece {
        name: addpiece_req.name,
        movements: addpiece_req.movements,
        description: addpiece_req.description,
    };

    // insert piece and get its ID
    let piece_id: i32 = diesel::insert_into(pieces::dsl::pieces)
        .values(&new_piece)
        .returning(pieces::dsl::id)
        .get_result(conn)
        .unwrap();

    // insert composer relationships
    for composer_id in addpiece_req.composer_ids {
        let _ = diesel::insert_into(piece_composers::table)
            .values((
                piece_composers::piece_id.eq(piece_id),
                piece_composers::composer_id.eq(composer_id),
            ))
            .execute(conn);
    }

    // insert songwriter relationships if they exist
    if let Some(songwriter_ids) = addpiece_req.songwriter_ids {
        for songwriter_id in songwriter_ids {
            let _ = diesel::insert_into(piece_songwriters::table)
                .values((
                    piece_songwriters::piece_id.eq(piece_id),
                    piece_songwriters::songwriter_id.eq(songwriter_id),
                ))
                .execute(conn);
        }
    }

    Response {
        success: true,
        message: String::new(),
    }
}

/// Add a release to the database
fn db_addrelease<T>(
    addrelease_req: AddReleaseRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::{release_performers, releases};

    // check for admin privileges
    let is_admin: bool = check_admin(addrelease_req.token, conn);
    if !is_admin {
        return Response {
            success: false,
            message: "User is not an admin".to_string(),
        };
    }

    // insert the new release into the database
    let new_release = insert::NewRelease {
        name: addrelease_req.name,
        description: addrelease_req.description,
        image_path: None,
    };

    // insert release and get its ID
    let release_id: i32 = diesel::insert_into(releases::dsl::releases)
        .values(&new_release)
        .returning(releases::dsl::id)
        .get_result(conn)
        .unwrap();

    // insert performer relationships
    for performer_id in addrelease_req.performer_ids {
        let _ = diesel::insert_into(release_performers::table)
            .values((
                release_performers::release_id.eq(release_id),
                release_performers::performer_id.eq(performer_id),
            ))
            .execute(conn);
    }

    // handle image path
    let mut new_image_path = String::new();
    if addrelease_req.has_image {
        new_image_path = format!("release-{}", release_id);
        diesel::update(releases::dsl::releases.find(release_id))
            .set(releases::dsl::image_path.eq(new_image_path.clone()))
            .execute(conn)
            .unwrap();
    }

    Response {
        success: true,
        message: new_image_path,
    }
}

/// Add a performer to the database, and return the image path of the performer if it exists
fn db_addperformer(
    addartist_req: AddArtistRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::performers;

    // insert the new performer into the database
    let new_performer = insert::NewPerformer {
        name: addartist_req.name,
        description: addartist_req.description,
        image_path: None,
    };
    let performer_id: i32 = diesel::insert_into(performers::dsl::performers)
        .values(&new_performer)
        .returning(performers::dsl::id)
        .get_result(conn)
        .unwrap();

    // actual image path is performer-[performer id]
    let mut new_image_path = String::new();
    if addartist_req.has_image {
        new_image_path = format!("performer-{}", performer_id);
        diesel::update(performers::dsl::performers.find(performer_id))
            .set(performers::dsl::image_path.eq(new_image_path.clone()))
            .execute(conn)
            .unwrap();
    }
    Response {
        success: true,
        message: new_image_path,
    }
}
/// Add a composer to the database, and return the image path of the composer if it exists
fn db_addcomposer(
    addartist_req: AddArtistRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::composers;

    // insert the new composer into the database
    let new_composer = insert::NewComposer {
        name: addartist_req.name,
        description: addartist_req.description,
        image_path: None,
    };
    let composer_id: i32 = diesel::insert_into(composers::dsl::composers)
        .values(&new_composer)
        .returning(composers::dsl::id)
        .get_result::<i32>(conn)
        .unwrap();

    // actual image path is composer-[composer id]
    let mut new_image_path = String::new();
    if addartist_req.has_image {
        new_image_path = format!("composer-{}", composer_id);
        diesel::update(composers::dsl::composers.find(composer_id))
            .set(composers::dsl::image_path.eq(new_image_path.clone()))
            .execute(conn)
            .unwrap();
    }
    Response {
        success: true,
        message: new_image_path,
    }
}

/// Add a songwriter to the database, and return the image path of the songwriter if it exists
fn db_addsongwriter(
    addartist_req: AddArtistRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::songwriters;

    // insert the new songwriter into the database
    let new_songwriter = insert::NewSongwriter {
        name: addartist_req.name,
        description: addartist_req.description,
        image_path: None,
    };
    let songwriter_id: i32 = diesel::insert_into(songwriters::dsl::songwriters)
        .values(&new_songwriter)
        .returning(songwriters::dsl::id)
        .get_result::<i32>(conn)
        .unwrap();

    // actual image path is songwriter-[songwriter id]
    let mut new_image_path = String::new();
    if addartist_req.has_image {
        new_image_path = format!("songwriter-{}", songwriter_id);
        diesel::update(songwriters::dsl::songwriters.find(songwriter_id))
            .set(songwriters::dsl::image_path.eq(new_image_path.clone()))
            .execute(conn)
            .unwrap();
    }
    Response {
        success: true,
        message: new_image_path,
    }
}

/// Add an artist to the database, and return the image path of the artist if it exists
fn db_addartist<T>(
    addartist_req: AddArtistRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    // check for admin privileges
    let is_admin: bool = check_admin(addartist_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: "User is not an admin".to_string(),
        };
    }

    // determine the type of artist to add
    if addartist_req.artist_type == "performer" {
        return db_addperformer(addartist_req, conn);
    } else if addartist_req.artist_type == "composer" {
        return db_addcomposer(addartist_req, conn);
    } else if addartist_req.artist_type == "songwriter" {
        return db_addsongwriter(addartist_req, conn);
    } else {
        return Response {
            success: false,
            message: "Invalid artist type".to_string(),
        };
    }
}

/// Add a piece to the database
#[post("/music/add/piece")]
pub async fn addpiece(
    addpiece_req: Json<AddPieceRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the addpiece response from database
    let mut conn = pool.get().expect("Connection pool error");
    let addpiece_response =
        web::block(move || db_addpiece::<String>(addpiece_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match addpiece_response {
        Ok(response) => {
            // handle case where server successfully processes the request
            HttpResponse::Created()
                .content_type("application/json")
                .json(response)
        }
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Add an releases to the database
#[post("/music/add/release")]
pub async fn addrelease(
    addrelease_req: Json<AddReleaseRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the addrelease response from database
    let mut conn = pool.get().expect("Connection pool error");
    let addrelease_response =
        web::block(move || db_addrelease::<String>(addrelease_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match addrelease_response {
        Ok(response) => {
            // handle case where server successfully processes the request
            HttpResponse::Created()
                .content_type("application/json")
                .json(response)
        }
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Add a recording to the database
#[post("/music/add/recording")]
pub async fn addrecording(
    addrecording_req: Json<AddRecordingRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the addrecording response from database
    println!("reached 1");
    let mut conn = pool.get().expect("Connection pool error");
    let addrecording_response =
        web::block(move || db_addrecording::<String>(addrecording_req.into_inner(), &mut conn))
            .await;

    println!("reached 2");

    // return the appropriate response and handle errors
    match addrecording_response {
        Ok(response) => {
            // handle case where server successfully processes the request
            HttpResponse::Created()
                .content_type("application/json")
                .json(response)
        }
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Add an artist to the database
#[post("/music/add/artist")]
pub async fn addartist(
    addartist_req: Json<AddArtistRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the addartist response from database
    let mut conn = pool.get().expect("Connection pool error");
    let addartist_response =
        web::block(move || db_addartist::<String>(addartist_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match addartist_response {
        Ok(response) => {
            // handle case where server successfully processes the request
            HttpResponse::Created()
                .content_type("application/json")
                .json(response)
        }
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}
