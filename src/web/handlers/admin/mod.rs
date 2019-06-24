pub mod user;

use rocket::get;
use web::guards::admin::Admin;
use rocket::response::{Redirect, Responder};

#[get("/admin")]
pub fn home(_admin: Admin) -> impl Responder<'static> {
    Redirect::to("/admin/users")
}
