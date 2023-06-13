// @generated automatically by Diesel CLI.

diesel::table! {
    balance (id) {
        id -> Integer,
        user_id -> Varchar,
        points -> Float,
    }
}

diesel::table! {
    options (id) {
        id -> Integer,
        guild_id -> Varchar,
        user_id -> Varchar,
        option -> Varchar,
        value -> Varchar,
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

diesel::table! {
    status (id) {
        id -> Integer,
        guild_id -> Varchar,
        user_id -> Varchar,
        since_online -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    balance,
    options,
    permissions,
    status,
);
