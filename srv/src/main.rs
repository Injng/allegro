use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, io};

pub mod api;
pub mod insert;
pub mod models;
pub mod schema;

/// Generic response to denote whether operation was successful
#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: T,
}

/// A generic id request from a user
#[derive(Deserialize, Serialize)]
pub struct IdRequest {
    pub id: i32,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // establish environment variables and enable logging
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    // connect to the database
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool<ConnectionManager<PgConnection>> = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // create a new API server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(api::auth::adduser)
            .service(api::auth::countuser)
            .service(api::auth::login)
            .service(api::addmusic::addartist)
            .service(api::addmusic::addpiece)
            .service(api::addmusic::addrecording)
            .service(api::addmusic::addrelease)
            .service(api::get::getcomposer)
            .service(api::get::getcomposers)
            .service(api::get::getperformer)
            .service(api::get::getperformers)
            .service(api::get::getpiece)
            .service(api::get::getpieces)
            .service(api::get::getrecording)
            .service(api::get::getrecordings)
            .service(api::get::getrelease)
            .service(api::get::getreleases)
            .service(api::get::getsongwriter)
            .service(api::get::getsongwriters)
            .service(api::search::searchcomposer)
            .service(api::search::searchperformer)
            .service(api::search::searchpiece)
            .service(api::search::searchrecording)
            .service(api::search::searchrelease)
            .service(api::search::searchsongwriter)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
