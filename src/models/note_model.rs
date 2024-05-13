use crate::models::{auth_model::User, category_model::Category};
use chrono::prelude::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize, Clone, Copy)]
#[ExistingTypePath = "crate::schema::sql_types::NoteContentType"]
pub enum NoteContentType {
    Markdown,
    Plaintext,
}

#[derive(
    Debug,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    AsChangeset,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(User, foreign_key = id))]
#[diesel(primary_key(id))]
#[diesel(table_name=crate::schema::note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Note {
    #[diesel(column_name = "id")]
    pub note_id: Uuid,

    #[diesel(column_name = "name")]
    pub note_name: String,

    #[diesel(column_name = "content")]
    pub note_content: String,

    #[diesel(column_name = "content_type")]
    pub note_content_type: NoteContentType,

    #[diesel(column_name = "archived")]
    pub note_archived: bool,

    #[diesel(column_name = "created_at")]
    pub note_created_at: NaiveDateTime,

    #[diesel(column_name = "updated_at")]
    pub note_updated_at: NaiveDateTime,

    #[diesel(column_name = "color")]
    pub note_color: String,

    #[diesel(column_name = "owner_id")]
    pub usr_id: Uuid,
}

#[derive(
    Debug,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    AsChangeset,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(Note, foreign_key = note_id))]
#[diesel(belongs_to(Category, foreign_key = cat_id))]
#[diesel(primary_key(id))]
#[diesel(table_name=crate::schema::note_has_category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NoteHasCategory {
    #[diesel(column_name = "id")]
    pub note_has_cat_id: Uuid,

    #[diesel(column_name = "note_id")]
    pub note_id: Uuid,

    #[diesel(column_name = "cat_id")]
    pub cat_id: Uuid,
}
