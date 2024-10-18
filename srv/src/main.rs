use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::{env, io};

pub mod api;
pub mod insert;
pub mod models;
pub mod schema;

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
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(api::auth::adduser)
            .service(api::auth::login)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
