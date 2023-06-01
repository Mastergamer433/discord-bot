// @generated automatically by Diesel CLI.

diesel::table! {
    balance (id) {
        id -> Integer,
        user_id -> Varchar,
        points -> Float,
    }
}
