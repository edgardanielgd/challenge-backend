use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    AsChangeset,
    Identifiable,
    Clone,
)]
#[diesel(belongs_to(crate::models::auth::User, foreign_key = usr_id))]
#[diesel(primary_key(id))]
#[diesel(table_name=crate::schema::category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    #[diesel(column_name = "id")]
    pub cat_id: Uuid,

    #[diesel(column_name = "name")]
    pub cat_name: String,

    #[diesel(column_name = "color")]
    pub cat_color: String,

    #[diesel(column_name = "owner_id")]
    pub user_id: Uuid,
}
