use rocket::catch;
use rocket::request::Request;
use rocket::response::{Redirect, Responder};

#[catch(401)]
pub fn unauthorized(_req: &Request) -> impl Responder<'static> {
    Redirect::to("/login")
}
