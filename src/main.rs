use catchers::error_catcher;
use controllers::{auth_controller, category_controller, note_controller};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod catchers;
mod controllers;
mod db;
mod guards;
mod models;
mod schema;
mod services;
mod validators;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    // Cors middleware to allow cross-origin requests
    // Reference: https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let rocket_instance = rocket::build();
    let rocket_instance = rocket_instance.attach(
        cors.to_cors()
            .expect("Error while attaching CORS fairing to Rocket instance"),
    );
    let rocket_instance =
        rocket_instance.register("/", catchers![rocket_validation::validation_catcher]);
    let rocket_instance = category_controller::mount(rocket_instance);
    let rocket_instance = note_controller::mount(rocket_instance);
    let rocket_instance = auth_controller::mount(rocket_instance);
    let rocket_instance = error_catcher::register(rocket_instance);
    let rocket_instance = rocket_instance.manage(db::create_connection());
    rocket_instance.launch().await.ok();
}
