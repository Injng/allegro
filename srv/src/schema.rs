// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
    }
}

diesel::table! {
    composers (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    performers (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    piece_composers (piece_id, composer_id) {
        piece_id -> Int4,
        composer_id -> Int4,
    }
}

diesel::table! {
    piece_songwriters (piece_id, songwriter_id) {
        piece_id -> Int4,
        songwriter_id -> Int4,
    }
}

diesel::table! {
    pieces (id) {
        id -> Int4,
        name -> Varchar,
        movements -> Nullable<Int4>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    recording_performers (recording_id, performer_id) {
        recording_id -> Int4,
        performer_id -> Int4,
    }
}

diesel::table! {
    recordings (id) {
        id -> Int4,
        piece_name -> Varchar,
        piece_id -> Int4,
        release_id -> Int4,
        track_number -> Int4,
        file_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    release_performers (release_id, performer_id) {
        release_id -> Int4,
        performer_id -> Int4,
    }
}

diesel::table! {
    releases (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    songwriters (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
        #[max_length = 255]
        session -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(piece_composers -> composers (composer_id));
diesel::joinable!(piece_composers -> pieces (piece_id));
diesel::joinable!(piece_songwriters -> pieces (piece_id));
diesel::joinable!(piece_songwriters -> songwriters (songwriter_id));
diesel::joinable!(recording_performers -> performers (performer_id));
diesel::joinable!(recording_performers -> recordings (recording_id));
diesel::joinable!(recordings -> pieces (piece_id));
diesel::joinable!(recordings -> releases (release_id));
diesel::joinable!(release_performers -> performers (performer_id));
diesel::joinable!(release_performers -> releases (release_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    composers,
    performers,
    piece_composers,
    piece_songwriters,
    pieces,
    recording_performers,
    recordings,
    release_performers,
    releases,
    songwriters,
    users,
);
