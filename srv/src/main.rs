use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

fn main() {
    // establish environment variables
    dotenv().ok();

    // connect to the database
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection: PgConnection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}
