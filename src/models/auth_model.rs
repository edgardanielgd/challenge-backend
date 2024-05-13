use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Deserialize, Queryable, Selectable, Insertable, Serialize, AsChangeset, Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name=crate::schema::app_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[diesel(column_name = "id")]
    pub usr_id: Uuid,

    #[diesel(column_name = "username")]
    pub usr_name: String,

    #[diesel(column_name = "email")]
    pub usr_email: String,

    #[diesel(column_name = "password")]
    pub usr_password: String,

    #[diesel(column_name = "created_at")]
    pub usr_created_at: chrono::NaiveDateTime,
}
