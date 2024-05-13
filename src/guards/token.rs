use crate::models::api::{
    auth_crud_model::{Claims, JWT},
    response_model::NetworkResponse,
};
use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use std::env;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse<String>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse<String>> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized("Unauthorized".to_string()),
            )),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(_) => Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized("Unauthorized".to_string()),
                )),
            },
        }
    }
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}

pub fn create_jwt(id: uuid::Uuid) -> Result<String, Error> {
    dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let expiration_time = match env::var("JWT_EXPIRATION_TIME_HOURS") {
        Ok(time) => time.parse::<i64>().unwrap(),
        Err(_) => 12,
    };

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(expiration_time))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
