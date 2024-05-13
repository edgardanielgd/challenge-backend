use crate::{
    models::{category_model::Category, note_model::NoteContentType},
    schema::note,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NoteCreate {
    #[validate(length(min = 1, max = 255))]
    pub note_name: String,

    #[validate(length(min = 1))]
    pub note_content: String,

    pub note_content_type: NoteContentType,

    pub note_archived: bool,

    #[validate(custom = "crate::validators::regex_validators::validate_hex_color")]
    pub note_color: String,
}

#[derive(Debug, Deserialize, AsChangeset, Validate)]
#[diesel(table_name = note)]
pub struct NoteUpdate {
    #[diesel(column_name = "name")]
    #[validate(length(min = 1, max = 255))]
    pub note_name: Option<String>,

    #[diesel(column_name = "content")]
    #[validate(length(min = 1))]
    pub note_content: Option<String>,

    #[diesel(column_name = "content_type")]
    pub note_content_type: Option<NoteContentType>,

    #[diesel(column_name = "archived")]
    pub note_archived: Option<bool>,

    #[diesel(column_name = "color")]
    #[validate(custom = "crate::validators::regex_validators::validate_hex_color")]
    pub note_color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteWithCategories {
    pub note_id: Uuid,
    pub note_name: String,
    pub note_content: String,
    pub note_content_type: NoteContentType,
    pub note_archived: bool,
    pub note_created_at: NaiveDateTime,
    pub note_updated_at: NaiveDateTime,
    pub note_color: String,
    pub usr_id: Uuid,

    pub categories: Vec<Category>,
}
