use crate::models::{Composer, Performer, Songwriter};
use crate::Response;

use actix_web::web::Data;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

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
