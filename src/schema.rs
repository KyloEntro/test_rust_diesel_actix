// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        comment -> Nullable<Text>,
        added -> Nullable<Timestamp>,
    }
}
