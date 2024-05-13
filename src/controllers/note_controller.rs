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
            category_crud_model::NoteCategoriesFilter,
            note_crud_model::{NoteCreate, NoteUpdate, NoteWithCategories},
            response_model::NetworkResponse::{self, Content, ServiceUnavailable},
        },
        note_model::{Note, NoteHasCategory},
    },
    services::note_service,
};

#[get("/<id>")]
pub async fn get_note_by_id(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    id: Uuid,
) -> Result<NetworkResponse<Json<NoteWithCategories>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let note = note_service::get_note_by_id(connection, id, user_id);

    if note.is_err() {
        return Err(ServiceUnavailable(note.err().unwrap()));
    }

    Ok(Content(Json(note.unwrap())))
}

// Intentionally using POST to collect categories array via body
#[post("/?<archived>&<unarchived>", data = "<categories>")]
pub async fn get_all_notes(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    categories: Option<Json<NoteCategoriesFilter>>,
    archived: Option<bool>,
    unarchived: Option<bool>,
) -> Result<NetworkResponse<Json<Vec<NoteWithCategories>>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;

    let categories = categories.map(|c| c.0.categories);

    let notes = note_service::get_notes(connection, user_id, archived, unarchived, categories);

    if notes.is_err() {
        return Err(ServiceUnavailable(notes.err().unwrap()));
    }

    Ok(Content(Json(notes.unwrap())))
}

#[post("/create", data = "<note>")]
pub async fn create_note(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    note: Validated<Json<NoteCreate>>,
) -> Result<NetworkResponse<Json<Note>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let note = note.0 .0;

    let note = note_service::create_note(connection, note, user_id);

    if note.is_err() {
        return Err(ServiceUnavailable(note.err().unwrap()));
    }

    Ok(Content(Json(note.unwrap())))
}

#[patch("/<id>", data = "<note>")]
pub async fn update_note(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    id: Uuid,
    note: Validated<Json<NoteUpdate>>,
) -> Result<NetworkResponse<Json<Note>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let note = note.0 .0;

    let note = note_service::update_note(connection, id, note, user_id);

    if note.is_err() {
        return Err(ServiceUnavailable(note.err().unwrap()));
    }

    Ok(Content(Json(note.unwrap())))
}

#[delete("/<id>")]
pub async fn delete_note(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    id: Uuid,
) -> Result<NetworkResponse<String>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let result = note_service::delete_note(connection, id, user_id);

    if result.is_err() {
        return Err(ServiceUnavailable(result.err().unwrap()));
    }

    Ok(Content("Note deleted".to_string()))
}

#[patch("/<note_id>/category/<cat_id>")]
pub async fn add_category_to_note(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    note_id: Uuid,
    cat_id: Uuid,
) -> Result<NetworkResponse<Json<NoteHasCategory>>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let note_has_category: Result<NoteHasCategory, String> =
        note_service::add_category_to_note(connection, note_id, cat_id, user_id);

    if note_has_category.is_err() {
        return Err(ServiceUnavailable(note_has_category.err().unwrap()));
    }

    Ok(Content(Json(note_has_category.unwrap())))
}

#[delete("/<note_id>/category/<cat_id>")]
pub async fn remove_category_from_note(
    connection: &State<db::PgPool>,
    jwt: Result<JWT, NetworkResponse<String>>,
    note_id: Uuid,
    cat_id: Uuid,
) -> Result<NetworkResponse<String>, NetworkResponse<String>> {
    if jwt.is_err() {
        return Err(jwt.err().unwrap());
    }
    let user_id = jwt.unwrap().claims.subject_id;
    let note_has_category =
        note_service::remove_category_from_note(connection, note_id, cat_id, user_id);

    if note_has_category.is_err() {
        return Err(ServiceUnavailable(note_has_category.err().unwrap()));
    }

    Ok(Content("Category removed from note".to_string()))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/notes",
        routes![
            get_all_notes,
            get_note_by_id,
            create_note,
            update_note,
            delete_note,
            add_category_to_note,
            remove_category_from_note
        ],
    )
}
