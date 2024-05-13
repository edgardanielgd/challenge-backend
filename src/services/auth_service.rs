use diesel::prelude::*;
use pwhash::sha512_crypt;
use rocket::State;
use uuid::Uuid;

use crate::{
    db::PgPool,
    guards::token::create_jwt,
    models::{
        api::auth_crud_model::{Token, UserData, UserLogin, UserRegister},
        auth_model::User,
    },
    schema::app_user,
};

pub fn login(connection: &State<PgPool>, data: UserLogin) -> Result<Token, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let user = app_user::table
        .filter(
            app_user::email
                .eq(&data.email_or_username)
                .or(app_user::username.eq(&data.email_or_username)),
        )
        .first::<User>(&mut connection)
        .map_err(|e| format!("Invalid credentials"))?;

    if !sha512_crypt::verify(&data.password, &user.usr_password) {
        return Err("Invalid credentials".to_string());
    }

    let token = create_jwt(user.usr_id);

    if token.is_err() {
        return Err("Failed to create token".to_string());
    }

    Ok(Token {
        access_token: token.unwrap(),
    })
}

pub fn register(connection: &State<PgPool>, data: UserRegister) -> Result<UserData, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let result: User = diesel::insert_into(app_user::table)
        .values(&User {
            usr_id: Uuid::new_v4(),
            usr_name: data.username,
            usr_email: data.email,
            usr_password: sha512_crypt::hash(&data.password).unwrap(),
            usr_created_at: chrono::Utc::now().naive_utc(),
        })
        .get_result::<User>(&mut connection)
        .map_err(|e| format!("Failed to create category: {}", e.to_string()))?;

    Ok(UserData {
        usr_id: result.usr_id,
        usr_name: result.usr_name,
        usr_email: result.usr_email,
        usr_created_at: result.usr_created_at,
    })
}

pub fn me(connection: &State<PgPool>, id: uuid::Uuid) -> Result<UserData, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let result = app_user::table
        .filter(app_user::id.eq(id))
        .first::<User>(&mut connection)
        .map_err(|e| format!("Failed to get user: {}", e.to_string()))?;

    Ok(UserData {
        usr_id: result.usr_id,
        usr_name: result.usr_name,
        usr_email: result.usr_email,
        usr_created_at: result.usr_created_at,
    })
}
