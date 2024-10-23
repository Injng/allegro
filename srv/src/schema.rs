// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        image_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    releases (id) {
        id -> Int4,
        name -> Varchar,
        artist_id -> Int4,
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

diesel::joinable!(releases -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    artists,
    releases,
    users,
);
