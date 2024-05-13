use diesel::prelude::*;
use rocket::State;
use uuid::Uuid;

use crate::{
    db::PgPool,
    models::{
        api::note_crud_model::{NoteCreate, NoteUpdate, NoteWithCategories},
        category_model::Category,
        note_model::{Note, NoteHasCategory},
    },
    schema::*,
};

pub fn get_notes(
    connection: &State<PgPool>,
    user_id: Uuid,
    archived: Option<bool>,
    unarchived: Option<bool>,
    categories: Option<Vec<Uuid>>,
) -> Result<Vec<NoteWithCategories>, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let mut query = note::table
        .left_join(note_has_category::table)
        .filter(note::owner_id.eq(user_id))
        .into_boxed();

    query = match (archived, unarchived) {
        (Some(true), _) => query.filter(note::archived.eq(true)),
        (_, Some(true)) => query.filter(note::archived.eq(false)),
        _ => query,
    };

    if let Some(categories) = categories {
        query = match categories.len() > 0 {
            true => query.filter(note_has_category::cat_id.eq_any(categories)),
            false => query,
        };
    }

    let notes: Vec<Note> = query
        .select(note::all_columns)
        .distinct()
        .order(note::created_at.desc())
        .load::<Note>(&mut connection)
        .map_err(|e| format!("Failed to load notes: {}", e.to_string()))?;

    let categories: Vec<(Uuid, Category)> = NoteHasCategory::belonging_to(&notes)
        .inner_join(category::table)
        .select((note_has_category::note_id, category::all_columns))
        .load::<(Uuid, Category)>(&mut connection)
        .map_err(|e| format!("Failed to load categories data: {}", e.to_string()))?;

    let grouped = categories.into_iter().fold(
        std::collections::HashMap::new(),
        |mut acc, (note_id, category)| {
            acc.entry(note_id).or_insert_with(Vec::new).push(category);
            acc
        },
    );

    let mut notes_with_categories = Vec::new();

    for note in notes {
        let note_categories = grouped.get(&note.note_id);

        notes_with_categories.push(NoteWithCategories {
            note_id: note.note_id,
            note_name: note.note_name,
            note_content: note.note_content,
            note_content_type: note.note_content_type,
            note_archived: note.note_archived,
            note_created_at: note.note_created_at,
            note_updated_at: note.note_updated_at,
            note_color: note.note_color,
            usr_id: note.usr_id,
            categories: note_categories
                .map(|categories| categories.clone())
                .unwrap_or_default()
                .to_vec(),
        });
    }

    Ok(notes_with_categories)
}

pub fn get_note_by_id(
    connection: &State<PgPool>,
    uuid: Uuid,
    user_id: Uuid,
) -> Result<NoteWithCategories, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let note = note::table
        .filter(note::owner_id.eq(user_id))
        .filter(note::id.eq(uuid))
        .first::<Note>(&mut connection)
        .map_err(|e| format!("Failed to load note: {}", e.to_string()))?;

    let categories: Vec<Category> = NoteHasCategory::belonging_to(&note)
        .inner_join(category::table)
        .select(category::all_columns)
        .load::<Category>(&mut connection)
        .map_err(|e| format!("Failed to load categories: {}", e.to_string()))?;

    Ok(NoteWithCategories {
        note_id: note.note_id,
        note_name: note.note_name,
        note_content: note.note_content,
        note_content_type: note.note_content_type,
        note_archived: note.note_archived,
        note_created_at: note.note_created_at,
        note_updated_at: note.note_updated_at,
        note_color: note.note_color,
        usr_id: note.usr_id,
        categories,
    })
}

pub fn create_note(
    connection: &State<PgPool>,
    note: NoteCreate,
    user_id: Uuid,
) -> Result<Note, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let result: Note = diesel::insert_into(note::table)
        .values(&Note {
            usr_id: user_id,
            note_id: Uuid::new_v4(),
            note_name: note.note_name,
            note_content: note.note_content,
            note_content_type: note.note_content_type,
            note_archived: note.note_archived,
            note_color: note.note_color,
            note_created_at: chrono::Utc::now().naive_utc(),
            note_updated_at: chrono::Utc::now().naive_utc(),
        })
        .get_result(&mut connection)
        .map_err(|e| format!("Failed to create note: {}", e.to_string()))?;

    Ok(result)
}

pub fn update_note(
    connection: &State<PgPool>,
    id: Uuid,
    note: NoteUpdate,
    user_id: Uuid,
) -> Result<Note, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    let result = diesel::update(note::table)
        .filter(note::owner_id.eq(user_id))
        .filter(note::id.eq(id))
        .set(&note)
        .get_result(&mut connection)
        .map_err(|e| format!("Failed to update note: {}", e.to_string()))?;

    Ok(result)
}

pub fn delete_note(connection: &State<PgPool>, id: Uuid, user_id: Uuid) -> Result<(), String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    diesel::delete(note::table)
        .filter(note::owner_id.eq(user_id))
        .filter(note::id.eq(id))
        .execute(&mut connection)
        .map_err(|e| format!("Failed to delete category: {}", e.to_string()))?;

    Ok(())
}

pub fn add_category_to_note(
    connection: &State<PgPool>,
    note_id: Uuid,
    cat_id: Uuid,
    user_id: Uuid,
) -> Result<NoteHasCategory, String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    // Check if user has access to note and category
    category::table
        .filter(category::owner_id.eq(user_id))
        .filter(category::id.eq(cat_id))
        .first::<Category>(&mut connection)
        .map_err(|e| format!("Failed to load category: {}", e.to_string()))?;

    note::table
        .filter(note::owner_id.eq(user_id))
        .filter(note::id.eq(note_id))
        .first::<Note>(&mut connection)
        .map_err(|e| format!("Failed to load note: {}", e.to_string()))?;

    // Check if note already has category
    let note_has_category = note_has_category::table
        .filter(note_has_category::note_id.eq(note_id))
        .filter(note_has_category::cat_id.eq(cat_id))
        .first::<NoteHasCategory>(&mut connection)
        .optional()
        .map_err(|e| format!("Failed to load note_has_category: {}", e.to_string()))?;

    if note_has_category.is_some() {
        return Err("Note already has category".to_string());
    }

    let result: NoteHasCategory = diesel::insert_into(note_has_category::table)
        .values(&NoteHasCategory {
            note_has_cat_id: Uuid::new_v4(),
            note_id,
            cat_id,
        })
        .get_result(&mut connection)
        .map_err(|e| format!("Failed to create category from note: {}", e.to_string()))?;

    Ok(result)
}

pub fn remove_category_from_note(
    connection: &State<PgPool>,
    note_id: Uuid,
    cat_id: Uuid,
    user_id: Uuid,
) -> Result<(), String> {
    let mut connection = connection
        .get()
        .map_err(|e| format!("Failed to get connection: {}", e.to_string()))?;

    // Check if user has access to note and category
    category::table
        .filter(category::owner_id.eq(user_id))
        .filter(category::id.eq(cat_id))
        .first::<Category>(&mut connection)
        .map_err(|e| format!("Failed to load category: {}", e.to_string()))?;

    note::table
        .filter(note::owner_id.eq(user_id))
        .filter(note::id.eq(note_id))
        .first::<Note>(&mut connection)
        .map_err(|e| format!("Failed to load note: {}", e.to_string()))?;

    diesel::delete(note_has_category::table)
        .filter(note_has_category::note_id.eq(note_id))
        .filter(note_has_category::cat_id.eq(cat_id))
        .execute(&mut connection)
        .map_err(|e| format!("Failed to remove category from note: {}", e.to_string()))?;

    Ok(())
}
