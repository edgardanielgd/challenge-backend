use rocket::Responder;

#[derive(Responder, Debug)]
pub enum NetworkResponse<T> {
    #[response(status = 200)]
    Content(T),

    #[response(status = 201)]
    Created(T),

    #[response(status = 200)]
    Accepted(T),

    #[response(status = 400)]
    BadRequest(String),

    #[response(status = 401)]
    Unauthorized(String),

    #[response(status = 403)]
    Forbidden(String),

    #[response(status = 404)]
    NotFound(String),

    #[response(status = 409)]
    Conflict(String),

    #[response(status = 500)]
    InternalServerError(String),

    #[response(status = 503)]
    ServiceUnavailable(String),
}
