// @generated automatically by Diesel CLI.

diesel::table! {
    balance (id) {
        id -> Integer,
        user_id -> Varchar,
        points -> Float,
    }
}

diesel::table! {
    permissions (id) {
        id -> Integer,
        guild_id -> Varchar,
        user_id -> Varchar,
        permission_string -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    balance,
    permissions,
);
