use rocket::{Build, Request, Rocket};

#[catch(500)]
fn internal_error(_req: &Request) -> &'static str {
    "Internal Server Error"
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "Data Not Found"
}

#[catch(400)]
fn bad_request(_req: &Request) -> &'static str {
    "Bad Request"
}

#[catch(401)]
fn unauthorized(_req: &Request) -> &'static str {
    "Unauthorized"
}

pub fn register(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.register(
        "/",
        catchers![internal_error, not_found, bad_request, unauthorized],
    )
}
