use crate::models::{Composer, Performer, Piece, Release, Songwriter};
use crate::{IdRequest, Response};

use actix_web::web::{Data, Json};
use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

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
    use crate::schema::releases;

    // get the release from the database
    let release_res = releases::dsl::releases
        .filter(releases::dsl::id.eq(release_req.id))
        .first::<Release>(conn);
    let release: Release = match release_res {
        Ok(_) => release_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Release::new(),
            };
        }
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
    use crate::schema::pieces;

    // get the piece from the database
    let piece_res = pieces::dsl::pieces
        .filter(pieces::dsl::id.eq(piece_req.id))
        .first::<Piece>(conn);
    let piece: Piece = match piece_res {
        Ok(_) => piece_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Piece::new(),
            };
        }
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
    use crate::schema::pieces;

    // get all performers from the database
    let pieces_res = pieces::dsl::pieces.load::<Piece>(conn);
    let piece: Vec<Piece> = match pieces_res {
        Ok(_) => pieces_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: piece,
    }
}

/// Get all releases in the releases index
fn db_getreleases<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Release>> {
    use crate::schema::releases;

    // get all releases from the database
    let releases_res = releases::dsl::releases.load::<Release>(conn);
    let release: Vec<Release> = match releases_res {
        Ok(_) => releases_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: release,
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
    let getpieces_response = web::block(move || db_getpieces::<Vec<Piece>>(&mut conn)).await;

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
        web::block(move || db_getpiece::<Piece>(piece_req.into_inner(), &mut conn)).await;

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
    let getreleases_response = web::block(move || db_getreleases::<Vec<Release>>(&mut conn)).await;

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
        web::block(move || db_getrelease::<Release>(release_req.into_inner(), &mut conn)).await;

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
