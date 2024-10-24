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

/// Represents a new composer to insert into the composers table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::composers)]
pub struct NewComposer {
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

/// Represents a new performer to insert into the performers table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::performers)]
pub struct NewPerformer {
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

/// Represents a new songwriter to insert into the songwriters table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::songwriters)]
pub struct NewSongwriter {
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

/// Represents a new release to insert into the releases table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::releases)]
pub struct NewRelease {
    pub name: String,
    pub performer_id: i32,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

/// Represents a new piece to insert into the pieces table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::pieces)]
pub struct NewPiece {
    pub name: String,
    pub movements: Option<i32>,
    pub composer_id: i32,
    pub songwriter_id: Option<i32>,
    pub description: Option<String>,
}
