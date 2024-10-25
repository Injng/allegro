use crate::api::get::{Piece, Recording, Release};
use crate::models::{
    Admin, Composer, DbPiece, DbRecording, DbRelease, Performer, Songwriter, User,
};

use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use serde::{Deserialize, Serialize};

/// Generic response to denote whether operation was successful
#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: T,
}

/// A search request with a search query
#[derive(Deserialize, Serialize)]
pub struct SearchRequest {
    pub query: String,
    pub token: String,
}

impl SearchRequest {
    /// Convert the search query into a query that can return results with words between the search terms
    fn to_query(&self) -> String {
        return format!(
            "%{}%",
            self.query
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("%")
        );
    }
}

/// Given a token, return whether the user is an admin
fn check_admin(
    token: String,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> bool {
    use crate::schema::admin;
    use crate::schema::users;

    // check if the token is valid
    let user_res = users::dsl::users
        .filter(users::dsl::session.eq(Some(token)))
        .first::<User>(conn);
    let user: User = match user_res {
        Ok(_) => user_res.unwrap(),
        Err(_) => {
            return false;
        }
    };

    // check if the user is an admin
    let admin_user_res = admin::dsl::admin
        .filter(admin::dsl::username.eq(&user.username))
        .first::<Admin>(conn);
    match admin_user_res {
        Ok(_) => (),
        Err(_) => {
            return false;
        }
    };

    return true;
}

/// Search for a recording in the index and return a list of relevant recordings
fn db_searchrecording<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Recording>> {
    use crate::schema::{recording_performers, recordings};

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the recording in the database
    let recording_res = recordings::dsl::recordings
        .filter(recordings::dsl::piece_name.ilike(search_req.to_query()))
        .load::<DbRecording>(conn);
    let db_recordings: Vec<DbRecording> = match recording_res {
        Ok(_) => recording_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    // for each recording, get the performers
    let mut recordings: Vec<Recording> = Vec::new();
    for db_recording in db_recordings {
        // get all performer IDs for this recording
        let performer_ids: Vec<i32> = recording_performers::dsl::recording_performers
            .filter(recording_performers::dsl::recording_id.eq(db_recording.id))
            .select(recording_performers::dsl::performer_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // construct the full recording object
        let recording = Recording {
            id: db_recording.id,
            piece_id: db_recording.piece_id,
            release_id: db_recording.release_id,
            performer_ids,
            track_number: db_recording.track_number,
        };

        // add the recording to the list of recordings
        recordings.push(recording);
    }

    return Response {
        success: true,
        message: recordings,
    };
}

/// Search for a release in the index and return a list of relevant releases
fn db_searchrelease<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Release>> {
    use crate::schema::{recordings, release_performers, releases};

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the release in the database
    let release_res = releases::dsl::releases
        .filter(releases::dsl::name.ilike(search_req.to_query()))
        .load::<DbRelease>(conn);
    let db_releases: Vec<DbRelease> = match release_res {
        Ok(_) => release_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    // for each release, get the performers and recordings
    let mut releases: Vec<Release> = Vec::new();
    for db_release in db_releases {
        // get all performer IDs for this release
        let performer_ids: Vec<i32> = release_performers::dsl::release_performers
            .filter(release_performers::dsl::release_id.eq(db_release.id))
            .select(release_performers::dsl::performer_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // get IDs of all recordings on this release, if they exist
        let recording_ids: Vec<i32> = recordings::dsl::recordings
            .filter(recordings::dsl::release_id.eq(db_release.id))
            .select(recordings::dsl::id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // construct the full release object
        let release = Release {
            id: db_release.id,
            name: db_release.name,
            description: db_release.description,
            image_path: db_release.image_path,
            recording_ids: if recording_ids.is_empty() {
                None
            } else {
                Some(recording_ids)
            },
            performer_ids,
        };

        // push to the list of releases
        releases.push(release);
    }

    return Response {
        success: true,
        message: releases,
    };
}

/// Search for a piece in the index and return a list of relevant pieces
fn db_searchpiece<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Piece>> {
    use crate::schema::{piece_composers, piece_songwriters, pieces};

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the piece in the database
    let piece_res = pieces::dsl::pieces
        .filter(pieces::dsl::name.ilike(search_req.to_query()))
        .load::<DbPiece>(conn);
    let db_pieces: Vec<DbPiece> = match piece_res {
        Ok(_) => piece_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    // for each piece, get the composers, songwriters, and performers
    let mut pieces: Vec<Piece> = Vec::new();
    for db_piece in db_pieces {
        // get all composer IDs for this piece
        let composer_ids: Vec<i32> = piece_composers::dsl::piece_composers
            .filter(piece_composers::dsl::piece_id.eq(db_piece.id))
            .select(piece_composers::dsl::composer_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // get all songwriter IDs for this piece
        let songwriter_ids: Vec<i32> = piece_songwriters::dsl::piece_songwriters
            .filter(piece_songwriters::dsl::piece_id.eq(db_piece.id))
            .select(piece_songwriters::dsl::songwriter_id)
            .load::<i32>(conn)
            .unwrap_or_default();

        // construct the full piece object
        let piece = Piece {
            id: db_piece.id,
            name: db_piece.name,
            movements: db_piece.movements,
            description: db_piece.description,
            composer_ids,
            songwriter_ids: if songwriter_ids.is_empty() {
                None
            } else {
                Some(songwriter_ids)
            },
        };

        // push to the list of pieces
        pieces.push(piece);
    }

    Response {
        success: true,
        message: pieces,
    }
}

/// Search a songwriter in the songwriters index, and return a list of relevant songwriters
fn db_searchsongwriter<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Songwriter>> {
    use crate::schema::songwriters;

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the songwriter in the database
    let songwriter_res = songwriters::dsl::songwriters
        .filter(songwriters::dsl::name.ilike(search_req.to_query()))
        .load::<Songwriter>(conn);
    let songwriter: Vec<Songwriter> = match songwriter_res {
        Ok(_) => songwriter_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: songwriter,
    }
}

/// Search a composer in the composers index, and return a list of relevant composers
fn db_searchcomposer<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Composer>> {
    use crate::schema::composers;

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the composer in the database
    let composer_res = composers::dsl::composers
        .filter(composers::dsl::name.ilike(search_req.to_query()))
        .load::<Composer>(conn);
    let composer: Vec<Composer> = match composer_res {
        Ok(_) => composer_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: composer,
    }
}

/// Search a performer in the performers index, and return a list of relevant performers
fn db_searchperformer<T>(
    search_req: SearchRequest,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Response<Vec<Performer>> {
    use crate::schema::performers;

    // check for admin privileges
    let is_admin: bool = check_admin(search_req.token.clone(), conn);
    if !is_admin {
        return Response {
            success: false,
            message: Vec::new(),
        };
    }

    // search for the performer in the database
    let performer_res = performers::dsl::performers
        .filter(performers::dsl::name.ilike(search_req.to_query()))
        .load::<Performer>(conn);
    let performer: Vec<Performer> = match performer_res {
        Ok(_) => performer_res.unwrap(),
        Err(_) => {
            return Response {
                success: false,
                message: Vec::new(),
            };
        }
    };

    Response {
        success: true,
        message: performer,
    }
}

/// Search for an artist
#[post("/music/search/performer")]
pub async fn searchperformer(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchperformer response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchartist_response = web::block(move || {
        db_searchperformer::<Vec<Performer>>(search_req.into_inner(), &mut conn)
    })
    .await;

    // return the appropriate response and handle errors
    match searchartist_response {
        Ok(response) => {
            // handle case where server successfully processes the request
            HttpResponse::Created()
                .content_type("application/json")
                .json(response)
        }
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Search for a composer
#[post("/music/search/composer")]
pub async fn searchcomposer(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchcomposer response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchcomposer_response =
        web::block(move || db_searchcomposer::<Vec<Composer>>(search_req.into_inner(), &mut conn))
            .await;

    // return the appropriate response and handle errors
    match searchcomposer_response {
        Ok(response) => HttpResponse::Created()
            // handle case where server successfully processes the request
            .content_type("application/json")
            .json(response),
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Search for a songwriter
#[post("/music/search/songwriter")]
pub async fn searchsongwriter(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchsongwriter response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchsongwriter_response = web::block(move || {
        db_searchsongwriter::<Vec<Songwriter>>(search_req.into_inner(), &mut conn)
    })
    .await;

    // return the appropriate response and handle errors
    match searchsongwriter_response {
        Ok(response) => HttpResponse::Created()
            // handle case where server successfully processes the request
            .content_type("application/json")
            .json(response),
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Search for a piece
#[post("/music/search/piece")]
pub async fn searchpiece(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchpiece response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchpiece_response =
        web::block(move || db_searchpiece::<Vec<Piece>>(search_req.into_inner(), &mut conn)).await;

    // return the appropriate response and handle errors
    match searchpiece_response {
        Ok(response) => HttpResponse::Created()
            // handle case where server successfully processes the request
            .content_type("application/json")
            .json(response),
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Search for a release
#[post("/music/search/release")]
pub async fn searchrelease(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchrelease response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchrelease_response =
        web::block(move || db_searchrelease::<Vec<Release>>(search_req.into_inner(), &mut conn))
            .await;

    // return the appropriate response and handle errors
    match searchrelease_response {
        Ok(response) => HttpResponse::Created()
            // handle case where server successfully processes the request
            .content_type("application/json")
            .json(response),
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Search for a recording
#[post("/music/search/recording")]
pub async fn searchrecording(
    search_req: Json<SearchRequest>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // get the searchrecording response from database
    let mut conn = pool.get().expect("Connection pool error");
    let searchrecording_response = web::block(move || {
        db_searchrecording::<Vec<Recording>>(search_req.into_inner(), &mut conn)
    })
    .await;

    // return the appropriate response and handle errors
    match searchrecording_response {
        Ok(response) => HttpResponse::Created()
            // handle case where server successfully processes the request
            .content_type("application/json")
            .json(response),
        _ => {
            // handle case where server error occurs
            HttpResponse::InternalServerError().finish()
        }
    }
}
