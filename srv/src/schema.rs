/*
 * schema.rs
 * 
 * This file defines the database schema for the allegro database using Diesel ORM.
 * It specifies the structure and data types for each column in the database tables,
 * This schema is used by Diesel to generate SQL and interact with the database.
 */

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        session -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}
