use crate::models::Artist;

use actix_web::web::Data;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Deserialize, Serialize};

/// Generic response to denote whether operation was successful
#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: T,
}

/// Get all artists in the artists index
fn db_getartists<T>(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Artist>> {
    use crate::schema::artists;

    // get all artists from the database
    let artist_res = artists::dsl::artists.load::<Artist>(conn);
    let artist: Vec<Artist> = match artist_res {
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

/// Get all artists
#[get("/music/get/artists")]
pub async fn getartists(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the getartists response from database
    let mut conn = pool.get().expect("Connection pool error");
    let getartists_response = web::block(move || db_getartists::<Vec<Artist>>(&mut conn)).await;

    // return the appropriate response and handle errors
    match getartists_response {
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
