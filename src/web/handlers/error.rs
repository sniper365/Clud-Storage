use rocket::catch;
use rocket::request::Request;
use rocket::response::{Redirect, Responder};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

#[derive(Serialize)]
struct ErrorContext {}

#[catch(400)]
pub fn bad_request(_req: &Request) -> impl Responder<'static> {
    let context = ErrorContext {};

    Template::render("error/bad_request", &context)
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> impl Responder<'static> {
    Redirect::to("/login")
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> impl Responder<'static> {
    let context = ErrorContext {};

    Template::render("error/forbidden", &context)
}

#[catch(404)]
pub fn not_found(_req: &Request) -> impl Responder<'static> {
    let context = ErrorContext {};

    Template::render("error/not_found", &context)
}

#[catch(500)]
pub fn internal_server_error(_req: &Request) -> impl Responder<'static> {
    let context = ErrorContext {};

    Template::render("error/internal_server_error", &context)
}
