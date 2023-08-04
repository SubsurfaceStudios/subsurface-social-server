// @generated automatically by Diesel CLI.

diesel::table! {
    authentication (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
        hash -> Bytea,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Nullable<Text>,
        body -> Text,
        author -> Uuid,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Nullable<Timestamp>,
        attachments -> Jsonb,
    }
}

diesel::joinable!(posts -> authentication (author));

diesel::allow_tables_to_appear_in_same_query!(
    authentication,
    posts,
);
