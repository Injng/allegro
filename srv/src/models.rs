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
pub struct DbPiece {
    pub id: i32,
    pub name: String,
    pub movements: Option<i32>,
    pub description: Option<String>,
}

impl DbPiece {
    /// Create an empty piece
    pub fn new() -> Self {
        DbPiece {
            id: -1,
            name: "".to_string(),
            movements: None,
            description: None,
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::releases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbRelease {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

impl DbRelease {
    /// Create an empty release
    pub fn new() -> Self {
        DbRelease {
            id: -1,
            name: "".to_string(),
            description: None,
            image_path: None,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::piece_composers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PieceComposer {
    pub piece_id: i32,
    pub composer_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::piece_songwriters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PieceSongwriter {
    pub piece_id: i32,
    pub songwriter_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::release_performers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReleasePerformer {
    pub release_id: i32,
    pub performer_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::recordings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recording {
    pub id: i32,
    pub piece_id: i32,
    pub release_id: i32,
    pub file_path: String,
}

impl Recording {
    /// create an empty recording
    pub fn new() -> Self {
        Recording {
            id: -1,
            piece_id: -1,
            release_id: -1,
            file_path: "".to_string(),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::recording_performers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecordingPerformer {
    pub recording_id: i32,
    pub performer_id: i32,
}
