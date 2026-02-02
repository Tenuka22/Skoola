// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Text,
        user_id -> Text,
        refresh_token_hash -> Text,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        password_hash -> Text,
        role -> Text,
        google_id -> Nullable<Text>,
        github_id -> Nullable<Text>,
        is_verified -> Bool,
        verification_token -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        verification_sent_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(sessions, users,);
