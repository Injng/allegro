use chrono::NaiveDateTime;
use diesel::prelude::*;

/// Represents a new user to insert into the database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub session: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// Represents a new user to insert into the admin table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::admin)]
pub struct NewAdmin {
    pub username: String,
}

/// Represents a new artist to insert into the artists table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::artists)]
pub struct NewArtist {
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

/// Represents a new release to insert into the releases table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::releases)]
pub struct NewRelease {
    pub name: String,
    pub artist_id: i32,
    pub description: Option<String>,
    pub image_path: Option<String>,
}
