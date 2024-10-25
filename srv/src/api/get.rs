use crate::models::{Composer, DbPiece, DbRelease, Performer, Songwriter};
use crate::{IdRequest, Response};

use actix_web::web::{Data, Json};
use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Deserialize, Serialize};

/// Represents a release with all its associated data
#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub recording_ids: Option<Vec<i32>>,
    pub performer_ids: Vec<i32>,
}

impl Release {
    /// Create an empty release
    pub fn new() -> Self {
        Release {
            id: -1,
            name: "".to_string(),
            description: None,
            image_path: None,
            recording_ids: None,
            performer_ids: Vec::new(),
        }
    }
}

/// Represents a piece with all its associated data
#[derive(Debug, Deserialize, Serialize)]
pub struct Piece {
    pub id: i32,
    pub name: String,
    pub movements: Option<i32>,
    pub description: Option<String>,
    pub composer_ids: Vec<i32>,
    pub songwriter_ids: Option<Vec<i32>>,
}

impl Piece {
    /// Create an empty piece
    pub fn new() -> Self {
        Piece {
            id: -1,
            name: "".to_string(),
            movements: None,
            description: None,
            composer_ids: Vec::new(),
            songwriter_ids: None,
        }
    }
}

/// Get specific performer by id from the performers index
fn db_getperformer<T>(
    performer_req: IdRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Performer> {
    use crate::schema::performers;

    // get the performer from the database
    let performer_res = performers::dsl::performers
        .filter(performers::dsl::id.eq(performer_req.id))
        .first::<Performer>(conn);
    let performer: Performer = match performer_res {
        Ok(_) => performer_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Performer::new(),
            };
        }
    };

    Response {
        success: true,
        message: performer,
    }
}

/// Get specific composer by id from the composers index
fn db_getcomposer<T>(
    composer_req: IdRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Composer> {
    use crate::schema::composers;

    // get the composer from the database
    let composer_res = composers::dsl::composers
        .filter(composers::dsl::id.eq(composer_req.id))
        .first::<Composer>(conn);
    let composer: Composer = match composer_res {
        Ok(_) => composer_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Composer::new(),
            };
        }
    };

    Response {
        success: true,
        message: composer,
    }
}

/// Get specific songwriter by id from the songwriters index
fn db_getsongwriter<T>(
    songwriter_req: IdRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Songwriter> {
    use crate::schema::songwriters;

    // get the songwriter from the database
    let songwriter_res = songwriters::dsl::songwriters
        .filter(songwriters::dsl::id.eq(songwriter_req.id))
        .first::<Songwriter>(conn);
    let songwriter: Songwriter = match songwriter_res {
        Ok(_) => songwriter_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Songwriter::new(),
            };
        }
    };

    Response {
        success: true,
        message: songwriter,
    }
}

/// Get specific release by id from the releases index
fn db_getrelease<T>(
    release_req: IdRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Release> {
    use crate::schema::{recordings, release_performers, releases};

    // get the basic release data
    let db_release_res = releases::dsl::releases
        .filter(releases::dsl::id.eq(release_req.id))
        .first::<DbRelease>(conn);

    let db_release: DbRelease = match db_release_res {
        Ok(_) => db_release_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Release::new(),
            };
        }
    };

    // get all performer IDs for this release
    let performer_ids: Vec<i32> = release_performers::dsl::release_performers
        .filter(release_performers::dsl::release_id.eq(release_req.id))
        .select(release_performers::dsl::performer_id)
        .load::<i32>(conn)
        .unwrap_or_default();

    // get IDs of all recordings on this release, if they exist
    let recording_ids: Vec<i32> = recordings::dsl::recordings
        .filter(recordings::dsl::release_id.eq(release_req.id))
        .select(recordings::dsl::id)
        .load::<i32>(conn)
        .unwrap_or_default();

    // construct the full release object
    let release = Release {
        id: db_release.id,
        name: db_release.name,
        description: db_release.description,
        image_path: db_release.image_path,
        recording_ids: if recording_ids.is_empty() {
            None
        } else {
            Some(recording_ids)
        },
        performer_ids,
    };

    Response {
        success: true,
        message: release,
    }
}

/// Get specific piece by id from the pieces index
fn db_getpiece<T>(
    piece_req: IdRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Piece> {
    use crate::schema::{piece_composers, piece_songwriters, pieces};

    // Get the basic piece data
    let db_piece_res = pieces::dsl::pieces
        .filter(pieces::dsl::id.eq(piece_req.id))
        .first::<DbPiece>(conn);

    let db_piece: DbPiece = match db_piece_res {
        Ok(_) => db_piece_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Piece::new(),
            };
        }
    };

    // get all composer IDs for this piece
    let composer_ids: Vec<i32> = piece_composers::dsl::piece_composers
        .filter(piece_composers::dsl::piece_id.eq(piece_req.id))
        .select(piece_composers::dsl::composer_id)
        .load::<i32>(conn)
        .unwrap_or_default();

    // get all songwriter IDs for this piece
    let songwriter_ids: Vec<i32> = piece_songwriters::dsl::piece_songwriters
        .filter(piece_songwriters::dsl::piece_id.eq(piece_req.id))
        .select(piece_songwriters::dsl::songwriter_id)
        .load::<i32>(conn)
        .unwrap_or_default();

    // construct the full piece object
    let piece = Piece {
        id: db_piece.id,
        name: db_piece.name,
        movements: db_piece.movements,
        description: db_piece.description,
        composer_ids,
        songwriter_ids: if songwriter_ids.is_empty() {
            None
        } else {
            Some(songwriter_ids)
        },
    };

    Response {
        success: true,
        message: piece,
    }
}

/// Get all pieces in the pieces index
fn db_getpieces<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Piece>> {
    use crate::schema::{piece_composers, piece_songwriters, pieces};

    // get all basic piece data
    let db_pieces_res = pieces::dsl::pieces.load::<DbPiece>(conn);
    let db_pieces: Vec<DbPiece> = match db_pieces_res {
        Ok(_) => db_pieces_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    // convert each DB piece into a full Piece with related data
    let mut full_pieces: Vec<Piece> = Vec::new();
    for db_piece in db_pieces {
        // get composer IDs for this piece
        let composer_ids: Vec<i32> = piece_composers::dsl::piece_composers
            .filter(piece_composers::dsl::piece_id.eq(db_piece.id))
            .select(piece_composers::dsl::composer_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // get songwriter IDs for this piece
        let songwriter_ids: Vec<i32> = piece_songwriters::dsl::piece_songwriters
            .filter(piece_songwriters::dsl::piece_id.eq(db_piece.id))
            .select(piece_songwriters::dsl::songwriter_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        full_pieces.push(Piece {
            id: db_piece.id,
            name: db_piece.name,
            movements: db_piece.movements,
            description: db_piece.description,
            composer_ids,
            songwriter_ids: if songwriter_ids.is_empty() {
                None
            } else {
                Some(songwriter_ids)
            },
        });
    }

    Response {
        success: true,
        message: full_pieces,
    }
}

/// Get all releases in the releases index
fn db_getreleases<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Release>> {
    use crate::schema::{recordings, release_performers, releases};

    // get all basic release data
    let db_releases_res = releases::dsl::releases.load::<DbRelease>(conn);
    let db_releases: Vec<DbRelease> = match db_releases_res {
        Ok(_) => db_releases_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    // convert each DB release into a full Release with related data
    let mut full_releases: Vec<Release> = Vec::new();
    for db_release in db_releases {
        // get performer IDs for this release
        let performer_ids: Vec<i32> = release_performers::dsl::release_performers
            .filter(release_performers::dsl::release_id.eq(db_release.id))
            .select(release_performers::dsl::performer_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // get recording IDs for this release
        let recording_ids: Vec<i32> = recordings::dsl::recordings
            .filter(recordings::dsl::release_id.eq(db_release.id))
            .select(recordings::dsl::id)
            .load::<i32>(conn)
            .unwrap_or_default();

        full_releases.push(Release {
            id: db_release.id,
            name: db_release.name,
            description: db_release.description,
            image_path: db_release.image_path,
            recording_ids: if recording_ids.is_empty() {
                None
            } else {
                Some(recording_ids)
            },
            performer_ids,
        });
    }

    Response {
        success: true,
        message: full_releases,
    }
}

/// Get all performers in the performers index
fn db_getperformers<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Performer>> {
    use crate::schema::performers;

    // get all performers from the database
    let artist_res = performers::dsl::performers.load::<Performer>(conn);
    let artist: Vec<Performer> = match artist_res {
        Ok(_) => artist_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: artist,
    }
}

/// Get all composers in the composers index
fn db_getcomposers<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Composer>> {
    use crate::schema::composers;

    // get all composers from the database
    let artist_res = composers::dsl::composers.load::<Composer>(conn);
    let artist: Vec<Composer> = match artist_res {
        Ok(_) => artist_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: artist,
    }
}

/// Get all songwriters in the performers index
fn db_getsongwriters<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Songwriter>> {
    use crate::schema::songwriters;

    // get all songwriters from the database
    let artist_res = songwriters::dsl::songwriters.load::<Songwriter>(conn);
    let artist: Vec<Songwriter> = match artist_res {
        Ok(_) => artist_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: artist,
    }
}

/// Get all pieces
#[get("/music/get/pieces")]
pub async fn getpieces(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the getpieces response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getpieces_response = web::block(move || db_getpieces::<Vec<DbPiece>>(&mut conn)).await;

    // return the appropriate response and handle errors
    match getpieces_response {
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

/// Get specific piece
#[post("/music/get/piece")]
pub async fn getpiece(
    piece_req: Json<IdRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the getpiece response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getpiece_response =
        web::block(move || db_getpiece::<DbPiece>(piece_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match getpiece_response {
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

/// Get all releases
#[get("/music/get/releases")]
pub async fn getreleases(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the getreleases response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getreleases_response =
        web::block(move || db_getreleases::<Vec<DbRelease>>(&mut conn)).await;

    // return the appropriate response and handle errors
    match getreleases_response {
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

/// Get specific release
#[post("/music/get/release")]
pub async fn getrelease(
    release_req: Json<IdRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the getrelease response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getrelease_response =
        web::block(move || db_getrelease::<DbRelease>(release_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match getrelease_response {
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

/// Get all performers
#[get("/music/get/performers")]
pub async fn getperformers(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the getperformers response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getperformers_response =
        web::block(move || db_getperformers::<Vec<Performer>>(&mut conn)).await;

    // return the appropriate response and handle errors
    match getperformers_response {
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

/// Get specific performer
#[post("/music/get/performer")]
pub async fn getperformer(
    performer_req: Json<IdRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the getperformer response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getperformer_response =
        web::block(move || db_getperformer::<Performer>(performer_req.into_inner(), &mut conn))
            .await;

    // return the appropriate response and handle errors
    match getperformer_response {
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

/// Get all composers
#[get("/music/get/composers")]
pub async fn getcomposers(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the getcomposers response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getcomposers_response =
        web::block(move || db_getcomposers::<Vec<Performer>>(&mut conn)).await;

    // return the appropriate response and handle errors
    match getcomposers_response {
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

/// Get specific composer
#[post("/music/get/composer")]
pub async fn getcomposer(
    composer_req: Json<IdRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the getcomposer response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getcomposer_response =
        web::block(move || db_getcomposer::<Performer>(composer_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match getcomposer_response {
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

/// Get all songwriters
#[get("/music/get/songwriters")]
pub async fn getsongwriters(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the getsongwriters response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getsongwriters_response =
        web::block(move || db_getsongwriters::<Vec<Songwriter>>(&mut conn)).await;

    // return the appropriate response and handle errors
    match getsongwriters_response {
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

/// Get specific songwriter
#[post("/music/get/songwriter")]
pub async fn getsongwriter(
    songwriter_req: Json<IdRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the getsongwriter response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getsongwriter_response =
        web::block(move || db_getsongwriter::<Songwriter>(songwriter_req.into_inner(), &mut conn))
            .await;

    // return the appropriate response and handle errors
    match getsongwriter_response {
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
