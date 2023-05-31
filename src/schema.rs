// @generated automatically by Diesel CLI.

diesel::table! {
    balance (id) {
        id -> Integer,
        user_id -> Varchar,
        points -> Integer,
    }
}

diesel::table! {
    messages (id) {
        id -> Integer,
        message_id -> Varchar,
        user_id -> Varchar,
        channel_id -> Varchar,
        content -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    balance,
    messages,
);
