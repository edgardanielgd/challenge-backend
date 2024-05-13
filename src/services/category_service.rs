use diesel::prelude::*;
use rocket::State;
use uuid::Uuid;

use crate::{
    db::PgPool,
    models::{
        api::category_crud_model::{CategoryCreate, CategoryUpdate},
        category_model::Category,
    },
    schema::category,
};

pub fn get_categories(connection: &State<PgPool>, user_id: Uuid) -> Result<Vec<Category>, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let categories = category::table
        .filter(category::owner_id.eq(user_id))
        .load::<Category>(&mut connection)
        .map_err(|e| format!("Failed to load categories: {}", e.to_string()))?;
    Ok(categories)
}

pub fn get_categories_by_id(
    connection: &State<PgPool>,
    id: Uuid,
    user_id: Uuid,
) -> Result<Category, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let category = category::table
        .filter(category::owner_id.eq(user_id))
        .filter(category::id.eq(id))
        .first::<Category>(&mut connection)
        .map_err(|e| format!("Failed to load category: {}", e.to_string()))?;

    Ok(category)
}

pub fn create_category(
    connection: &State<PgPool>,
    category: CategoryCreate,
    user_id: Uuid,
) -> Result<Category, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let result: Category = diesel::insert_into(category::table)
        .values(&Category {
            cat_id: Uuid::new_v4(),
            user_id,
            cat_name: category.cat_name,
            cat_color: category.cat_color,
        })
        .get_result(&mut connection)
        .map_err(|e| format!("Failed to create category: {}", e.to_string()))?;

    Ok(result)
}

pub fn update_category(
    connection: &State<PgPool>,
    id: Uuid,
    category: CategoryUpdate,
    user_id: Uuid,
) -> Result<Category, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let result = diesel::update(category::table)
        .filter(category::owner_id.eq(user_id))
        .filter(category::id.eq(id))
        .set(&category)
        .get_result(&mut connection)
        .map_err(|e| format!("Failed to update category: {}", e.to_string()))?;

    Ok(result)
}

pub fn delete_category(connection: &State<PgPool>, id: Uuid, user_id: Uuid) -> Result<(), String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    diesel::delete(category::table)
        .filter(category::owner_id.eq(user_id))
        .filter(category::id.eq(id))
        .execute(&mut connection)
        .map_err(|e| format!("Failed to delete category: {}", e.to_string()))?;

    Ok(())
}
