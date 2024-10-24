use crate::models::{Admin, Performer, User};

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

/// Search a performer in the performers index, and return a list of relevant performers
fn db_searchperformer<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Performer>> {
    use crate::schema::performers;

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token, conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the performer in the database
    let search_query = format!("%{}%", search_req.query);
    let performer_res = performers::dsl::performers
        .filter(performers::dsl::name.ilike(search_query))
        .load::<Performer>(conn);
    let performer: Vec<Performer> = match performer_res {
        Ok(_) => performer_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: performer,
    }
}

/// Search for an artist
#[post("/music/search/performer")]
pub async fn searchperformer(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchperformer response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchartist_response = web::block(move || {
        db_searchperformer::<Vec<Performer>>(search_req.into_inner(), &mut conn)
    })
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
