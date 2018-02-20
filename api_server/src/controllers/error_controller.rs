use rocket::Request;
use rocket::Response;
use serde_json;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

#[derive(Serialize)]
struct ErrorResponse {
    pub status_code: i32,
    pub message: String
}

#[error(400)]
fn bad_request(_req: &Request) -> Response<'static> {
    let error = ErrorResponse {
        status_code: 400,
        message: String::from("Cannot or will not process due to malformed request")
    };

    let response = serde_json::to_string(&error).unwrap();

    Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[error(401)]
fn unauthorized(_req: &Request) -> Response<'static> {
    let error = ErrorResponse {
        status_code: 401,
        message: String::from("Authentication has failed or has not been provided")
    };

    let response = serde_json::to_string(&error).unwrap();

    Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[error(403)]
fn forbidden(_req: &Request) -> Response<'static> {
    let error = ErrorResponse {
        status_code: 403,
        message: String::from("Request was valid; client does not have permission to this resource")
    };

    let response = serde_json::to_string(&error).unwrap();

    Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[error(404)]
fn not_found(_req: &Request) -> Response<'static> {
    let error = ErrorResponse {
        status_code: 404,
        message: String::from("Resource could not be found.")
    };

    let response = serde_json::to_string(&error).unwrap();

    Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[error(500)]
fn internal_server_error(_req: &Request) -> Response<'static> {
    let error = ErrorResponse {
        status_code: 500,
        message: String::from("An unexpected error occured")
    };

    let response = serde_json::to_string(&error).unwrap();

    Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}
