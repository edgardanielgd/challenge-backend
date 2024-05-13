use rocket::{serde::json::Json, Build, Rocket, State};
use rocket_validation::Validated;

use crate::{
    db,
    models::api::{
        auth_crud_model::{Token, UserData, UserLogin, UserRegister, JWT},
        response_model::NetworkResponse::{self, BadRequest, Content},
    },
    services::auth_service,
};

#[post("/login", data = "<user>")]
pub async fn login(
    connection: &State<db::PgPool>,
    user: Validated<Json<UserLogin>>,
) -> Result<NetworkResponse<Json<Token>>, NetworkResponse<String>> {
    let user = user.0 .0;
    let user = auth_service::login(connection, user);

    if user.is_err() {
        return Err(BadRequest(user.err().unwrap()));
    }

    Ok(Content(Json(user.unwrap())))
}

#[post("/register", data = "<user>")]
pub async fn register(
    connection: &State<db::PgPool>,
    user: Validated<Json<UserRegister>>,
) -> Result<NetworkResponse<Json<UserData>>, NetworkResponse<String>> {
    let user = user.0 .0;
    let user = auth_service::register(connection, user);

    if user.is_err() {
        return Err(BadRequest(user.err().unwrap()));
    }

    Ok(Content(Json(user.unwrap())))
}

#[get("/me")]
pub async fn me(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
) -> Result<NetworkResponse<Json<UserData>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }

    let user_id = jwt.unwrap().claims.subject_id;

    let user = auth_service::me(connection, user_id);

    if user.is_err() {
        return Err(BadRequest(user.err().unwrap()));
    }

    Ok(Content(Json(user.unwrap())))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/auth", routes![login, register, me,])
}
