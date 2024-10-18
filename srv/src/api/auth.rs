use crate::insert;
use crate::models::{Admin, User};

use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error;
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use uuid::Uuid;

// constants for the PBKDF2 algorithm
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const PBKDF2_ITER: u32 = 100_000;
const HASH_LEN: usize = digest::SHA256_OUTPUT_LEN;

/// Generic response to denote whether operation was successful
#[derive(Deserialize, Serialize)]
pub struct Response {
    pub success: bool,
    pub message: String,
}

/// Return whether access is granted and a session token to an authorization request.
#[derive(Deserialize, Serialize)]
pub struct AuthResponse {
    pub access: bool,
    pub token: Option<String>,
}

/// An authorization request from a user.
#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

/// A request to create a new user from an administrator.
#[derive(Deserialize, Serialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
    pub token: String,
}

/// Represents session information returned from the server
#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub token: String,
    pub created_at: NaiveDateTime,
}

/// Add the user to the database, checking if admin privileges are required
fn db_adduser(
    adduser_req: AddUserRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response {
    use crate::schema::admin;
    use crate::schema::users;

    // if this is the first user, no admin privileges are required
    let num_users: i64 = users::dsl::users.count().get_result(conn).unwrap();
    let mut is_admin = true; // Make variable mutable with 'mut'
    if num_users == 0 {
        // Compare with reference
        is_admin = false;
    }

    // if admin privileges are required, proceed with auth checks
    if is_admin {
        // check if the token is valid
        let token = adduser_req.token;
        let user_res = users::dsl::users
            .filter(users::dsl::session.eq(Some(token)))
            .first::<User>(conn);
        let user: User = match user_res {
            Ok(_) => user_res.unwrap(),
            Err(_) => {
                return Response {
                    success: false,
                    message: "Invalid token".to_string(),
                }
            }
        };

        // check if the user is an admin
        let admin_user_res = admin::dsl::admin
            .filter(admin::dsl::username.eq(&user.username))
            .first::<Admin>(conn);
        match admin_user_res {
            Ok(_) => (),
            Err(_) => {
                return Response {
                    success: false,
                    message: "User is not an admin".to_string(),
                }
            }
        };
    }

    // generate a salt and hash the password with PBKDF2
    let salt: String = Uuid::new_v4().to_string();
    let mut password_hash: [u8; HASH_LEN] = [0u8; HASH_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(PBKDF2_ITER).unwrap(),
        &salt.as_bytes(),
        adduser_req.password.as_bytes(),
        &mut password_hash,
    );

    // insert the new user into the database
    let new_user = insert::NewUser {
        username: adduser_req.username,
        password_hash: String::from_utf8(password_hash.to_vec()).unwrap(),
        salt,
        session: None,
        created_at: None,
    };
    let _ = diesel::insert_into(users::dsl::users)
        .values(&new_user)
        .execute(conn);
    Response {
        success: true,
        message: "User sucessfully added".to_string(),
    }
}

/// Given a user's authentication request, check if the user exists and if the password is correct.
fn db_login(
    auth_req: AuthRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<AuthResponse, Error> {
    use crate::schema::users;

    // filter for the user
    let user_res = users::dsl::users
        .filter(users::dsl::username.eq(&auth_req.username))
        .first::<User>(conn);

    // check if user exists
    let user: User = match user_res {
        Ok(_) => user_res.unwrap(),
        Err(_) => {
            return Ok(AuthResponse {
                access: false,
                token: None,
            })
        }
    };

    // verify the password hash with PBKDF2
    let verify_res = pbkdf2::verify(
        PBKDF2_ALG,
        NonZeroU32::new(PBKDF2_ITER).unwrap(),
        &user.salt.as_bytes(),
        auth_req.password.as_bytes(),
        &user.password_hash.as_bytes(),
    );

    // if the password is correct, create a session token and set time created
    match verify_res {
        Ok(_) => {
            let token = Uuid::new_v4().to_string();
            let timestamp: NaiveDateTime = chrono::Utc::now().naive_utc();
            let _ = diesel::update(users::dsl::users)
                .filter(users::dsl::username.eq(&auth_req.username))
                .set((
                    users::dsl::session.eq(Some(token.clone())),
                    users::dsl::created_at.eq(Some(timestamp)),
                ))
                .execute(conn);
            Ok(AuthResponse {
                access: true,
                token: Some(token),
            })
        }
        Err(_) => Ok(AuthResponse {
            access: false,
            token: None,
        }),
    }
}

/// Add a user to the system, and determine if admin privileges are required
#[post("/auth/adduser")]
pub async fn adduser(
    adduser_req: Json<AddUserRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the adduser response from database
    let mut conn = pool.get().expect("Connection pool error");
    let adduser_response =
        web::block(move || db_adduser(adduser_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match adduser_response {
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

/// Determine if user is authorized, and if so return a session token.
#[post("/auth/login")]
pub async fn login(
    auth_req: Json<AuthRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the authorization response from database
    let mut conn = pool.get().expect("Connection pool error");
    let auth_response = web::block(move || db_login(auth_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match auth_response {
        Ok(Ok(response)) => {
            // handle case where login succeeds
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
