use crate::schema::category;
use diesel::prelude::*;

use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct NoteCategoriesFilter {
    pub categories: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CategoryCreate {
    #[validate(length(min = 1, max = 255))]
    pub cat_name: String,

    #[validate(custom = "crate::validators::regex_validators::validate_hex_color")]
    pub cat_color: String,
}

#[derive(Debug, Deserialize, AsChangeset, Validate)]
#[diesel(table_name = category)]
pub struct CategoryUpdate {
    #[diesel(column_name = "name")]
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[diesel(column_name = "color")]
    #[validate(custom = "crate::validators::regex_validators::validate_hex_color")]
    pub color: Option<String>,
}
