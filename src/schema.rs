// @generated automatically by Diesel CLI.

diesel::table! {
    balance (id) {
        id -> Integer,
        user_id -> Integer,
        points -> Float,
    }
}
