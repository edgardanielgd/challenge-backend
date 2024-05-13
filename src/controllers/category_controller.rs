use rocket::{
    serde::{json::Json, uuid::Uuid},
    Build, Rocket, State,
};
use rocket_validation::Validated;

use crate::{
    db,
    models::{
        api::{
            auth_crud_model::JWT,
            category_crud_model::{CategoryCreate, CategoryUpdate},
            response_model::NetworkResponse::{self, BadRequest, Content},
        },
        category_model::Category,
    },
    services::category_service,
};

#[get("/<id>")]
pub async fn get_category_by_id(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    id: Uuid,
) -> Result<NetworkResponse<Json<Category>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }

    let user_id = jwt.unwrap().claims.subject_id;
    let category = category_service::get_categories_by_id(connection, id, user_id);

    if category.is_err() {
        return Err(BadRequest(category.err().unwrap()));
    }

    Ok(Content(Json(category.unwrap())))
}

#[get("/")]
pub async fn get_all_categories(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
) -> Result<NetworkResponse<Json<Vec<Category>>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let categories: Result<Vec<Category>, String> =
        category_service::get_categories(connection, user_id);

    if categories.is_err() {
        return Err(BadRequest(categories.err().unwrap()));
    }

    Ok(Content(Json(categories.unwrap())))
}

#[post("/", data = "<category>")]
pub async fn create_category(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    category: Validated<Json<CategoryCreate>>,
) -> Result<NetworkResponse<Json<Category>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let category = category.0 .0;

    let category = category_service::create_category(connection, category, user_id);

    if category.is_err() {
        return Err(BadRequest(category.err().unwrap()));
    }

    Ok(Content(Json(category.unwrap())))
}

#[patch("/<id>", data = "<category>")]
pub async fn update_category(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    id: Uuid,
    category: Validated<Json<CategoryUpdate>>,
) -> Result<NetworkResponse<Json<Category>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let category = category.0 .0;

    let category = category_service::update_category(connection, id, category, user_id);

    if category.is_err() {
        return Err(BadRequest(category.err().unwrap()));
    }

    Ok(Content(Json(category.unwrap())))
}

#[delete("/<id>")]
pub async fn delete_category(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    id: Uuid,
) -> Result<NetworkResponse<String>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let result = category_service::delete_category(connection, id, user_id);

    if result.is_err() {
        return Err(BadRequest(result.err().unwrap()));
    }

    Ok(Content("Category deleted".to_string()))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/categories",
        routes![
            get_all_categories,
            get_category_by_id,
            create_category,
            update_category,
            delete_category
        ],
    )
}
