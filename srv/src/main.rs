use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::{env, io};

pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // establish environment variables
    dotenv().ok();

    // connect to the database
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // create a new API server
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
    }).bind("0.0.0.0:9000")?
        .run()
        .await
}
