use crate::models::User;

use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post, web};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct AuthResponse {
    pub access: bool,
    pub token: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub token: String,
    pub created_at: NaiveDateTime,
}

/// Given a user's authentication request, check if the user exists and if the password is correct.
fn db_login(auth_req: AuthRequest, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<AuthResponse, Error> {
    use crate::schema::users::dsl::*;

    // filter for the user
    let user = users
        .filter(username.eq(&auth_req.username))
        .first::<User>(conn);

    // if user exists, check password and create session token
    match user {
        Ok(user) => {
            // verify the password hash with PBKDF2
            let verify_res = pbkdf2::verify(
                pbkdf2::PBKDF2_HMAC_SHA256,
                NonZeroU32::new(100_000).unwrap(),
                &user.salt.as_bytes(),
                auth_req.password.as_bytes(),
                &user.password_hash.as_bytes(),
            );

            // if the password is correct, create a session token
            match verify_res {
                Ok(_) => {
                    let token = Uuid::new_v4().to_string();
                    let _ = diesel::update(users)
                        .filter(username.eq(&auth_req.username))
                        .set(session.eq(Some(token.clone())))
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
        Err(_) => Ok(AuthResponse {
            access: false,
            token: None,
        }),
    }
}

/// Determine if user is authorized, and if so return a session token.
#[post("/auth/login")]
pub async fn login(auth_req: Json<AuthRequest>, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // get the authorization response from database
    let mut conn = pool.get().expect("Connection pool error");
    let auth_response = web::block(move || db_login(auth_req.into_inner(), &mut conn))
        .await;

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

