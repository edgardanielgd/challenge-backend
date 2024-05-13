use rocket_validation::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct UserLogin {
    #[serde(alias = "identifier")]
    #[validate(length(min = 1))]
    pub email_or_username: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub usr_id: Uuid,
    pub usr_name: String,
    pub usr_email: String,
    pub usr_created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserRegister {
    #[validate(length(min = 7, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 50))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: Uuid,
    pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}
