use crate::insert;
use crate::models::{Admin, User};

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

/// An request to add an artist
#[derive(Deserialize, Serialize)]
pub struct AddArtistRequest {
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
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

/// Add an artist to the database, and return an id corresponding to the artist
fn db_addartist<T>(
    addartist_req: AddArtistRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<i32> {
    use crate::schema::artists;

    // check for admin privileges
    let is_admin: bool = check_admin(addartist_req.token, conn);
    if !is_admin {
        return Response {
            success: false,
            message: -1,
        };
    }

    // insert the new artist into the database
    let new_artist = insert::NewArtist {
        name: addartist_req.name,
        description: addartist_req.description,
        image_path: addartist_req.image_path,
    };
    let artist_id: i32 = diesel::insert_into(artists::dsl::artists)
        .values(&new_artist)
        .returning(artists::dsl::id)
        .get_result(conn)
        .unwrap();
    Response {
        success: true,
        message: artist_id,
    }
}

/// Add an artist to the database
#[post("/music/addartist")]
pub async fn adduser(
    addartist_req: Json<AddArtistRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the addartist response from database
    let mut conn = pool.get().expect("Connection pool error");
    let addartist_response =
        web::block(move || db_addartist::<i32>(addartist_req.into_inner(), &mut conn)).await;

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
