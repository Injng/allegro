use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub session: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::performers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Performer {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::releases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Release {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
    pub description: Option<String>,
    pub image_path: Option<String>,
}
