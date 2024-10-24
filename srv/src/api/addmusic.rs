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
    pub performer_id: i32,
    pub description: Option<String>,
    pub has_image: bool,
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

/// Add a release to the database, and return the image path of the release if it exists
fn db_addrelease<T>(
    addrelease_req: AddReleaseRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<String> {
    use crate::schema::releases;

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
        performer_id: addrelease_req.performer_id,
        description: addrelease_req.description,
        image_path: None,
    };
    let release_id: i32 = diesel::insert_into(releases::dsl::releases)
        .values(&new_release)
        .returning(releases::dsl::id)
        .get_result(conn)
        .unwrap();

    // actual image path is release-[release id]
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
            // handle case where server successfuly processes the request
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
            // handle case where server successfuly processes the request
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
