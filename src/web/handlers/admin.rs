use controllers::UserController;
use db::models::User;
use rocket::get;
use rocket::http::Status;
use rocket::response::{Redirect, Responder};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::admin::Admin;

#[derive(Serialize)]
pub struct IndexContext {
    user: User,
    users: Vec<User>,
}

#[get("/admin")]
pub fn index(admin: Admin) -> impl Responder<'static> {
    let user = admin.clone().user();

    let users = match UserController::index(user.clone()) {
        Ok(users) => users,
        Err(e) => return Err(Status::from(e)),
    };

    let context = IndexContext { user, users };

    Ok(Template::render("admin/index", &context))
}
