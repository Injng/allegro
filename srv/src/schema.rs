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
    pieces (id) {
        id -> Int4,
        name -> Varchar,
        movements -> Nullable<Int4>,
        composer_id -> Int4,
        songwriter_id -> Nullable<Int4>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    releases (id) {
        id -> Int4,
        name -> Varchar,
        performer_id -> Int4,
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

diesel::joinable!(pieces -> composers (composer_id));
diesel::joinable!(pieces -> songwriters (songwriter_id));
diesel::joinable!(releases -> performers (performer_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    composers,
    performers,
    pieces,
    releases,
    songwriters,
    users,
);
