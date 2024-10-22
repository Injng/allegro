use crate::models::{Admin, Artist, User};

use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Deserialize, Serialize};

/// Generic response to denote whether operation was successful
#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: T,
}

/// A search request with a search query
#[derive(Deserialize, Serialize)]
pub struct SearchRequest {
    pub query: String,
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

/// Search an artist in the artists index, and return a list of relevant artists
fn db_searchartist<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Artist>> {
    use crate::schema::artists;

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token, conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the artist in the database
    let search_query = format!("%{}%", search_req.query);
    let artist_res = artists::dsl::artists
        .filter(artists::dsl::name.ilike(search_query))
        .load::<Artist>(conn);
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

/// Search for an artist
#[post("/music/search/artist")]
pub async fn searchartist(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchartist response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchartist_response =
        web::block(move || db_searchartist::<Vec<Artist>>(search_req.into_inner(), &mut conn))
            .await;

    // return the appropriate response and handle errors
    match searchartist_response {
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
