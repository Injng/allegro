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

impl Performer {
    /// Create an empty performer
    pub fn new() -> Self {
        Performer {
            id: -1,
            name: "".to_string(),
            description: None,
            image_path: None,
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::composers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Composer {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

impl Composer {
    /// Create an empty composer
    pub fn new() -> Self {
        Composer {
            id: -1,
            name: "".to_string(),
            description: None,
            image_path: None,
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::songwriters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Songwriter {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

impl Songwriter {
    /// Create an empty songwriter
    pub fn new() -> Self {
        Songwriter {
            id: -1,
            name: "".to_string(),
            description: None,
            image_path: None,
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::pieces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Piece {
    pub id: i32,
    pub name: String,
    pub movements: Option<i32>,
    pub composer_id: i32,
    pub songwriter_id: Option<i32>,
    pub description: Option<String>,
}

impl Piece {
    /// Create an empty piece
    pub fn new() -> Self {
        Piece {
            id: -1,
            name: "".to_string(),
            movements: None,
            composer_id: -1,
            songwriter_id: None,
            description: None,
        }
    }
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

impl Release {
    /// Create an empty release
    pub fn new() -> Self {
        Release {
            id: -1,
            name: "".to_string(),
            performer_id: -1,
            description: None,
            image_path: None,
        }
    }
}
