// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "note_content_type"))]
    pub struct NoteContentType;
}

diesel::table! {
    app_user (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    category (id) {
        id -> Uuid,
        name -> Text,
        color -> Text,
        owner_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NoteContentType;

    note (id) {
        id -> Uuid,
        name -> Text,
        content -> Text,
        content_type -> NoteContentType,
        archived -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        color -> Text,
        owner_id -> Uuid,
    }
}

diesel::table! {
    note_has_category (id) {
        id -> Uuid,
        note_id -> Uuid,
        cat_id -> Uuid,
    }
}

diesel::joinable!(category -> app_user (owner_id));
diesel::joinable!(note -> app_user (owner_id));
diesel::joinable!(note_has_category -> category (cat_id));
diesel::joinable!(note_has_category -> note (note_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    category,
    note,
    note_has_category,
);
