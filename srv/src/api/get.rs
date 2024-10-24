use crate::models::Performer;
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

    // get all artists from the database
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
