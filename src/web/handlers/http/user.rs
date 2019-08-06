use controllers::UserController;
use db::models::User;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::auth::Auth;

#[derive(Serialize)]
pub struct IndexContext {
    user: User,
    users: Vec<User>,
}

#[get("/users")]
pub fn index(auth: Auth) -> impl Responder<'static> {
    let user = auth.clone().user();

    let users = match UserController::index(user.clone()) {
        Ok(users) => users,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    let context = IndexContext { user, users };

    Ok(Template::render("user/index", &context))
}

#[derive(Serialize)]
pub struct ShowContext {
    user: User,
    show: User,
}

#[get("/users/<user_id>")]
pub fn show(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let show = match UserController::show(user.clone(), user_id) {
        Ok(user) => user,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    let context = ShowContext { user, show };

    Ok(Template::render("user/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
}

#[get("/users/create")]
pub fn create(auth: Auth) -> impl Responder<'static> {
    let user = auth.clone().user();

    if let Err(e) = UserController::create(user.clone()) {
        return Err(Status::from(e));
    }

    let context = CreateContext { user };

    Ok(Template::render("user/create", &context))
}

#[derive(FromForm)]
pub struct StorePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users", data = "<payload>")]
pub fn store(auth: Auth, payload: Form<StorePayload>) -> impl Responder<'static> {
    let user = auth.clone().user();

    let user = match UserController::store(
        user.clone(),
        payload.name.clone(),
        payload.email.clone(),
        "guest".to_string(),
        payload.password.clone(),
    ) {
        Ok(user) => user,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    Ok(Redirect::to(format!("/users/{}", user.id())))
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    edit: User,
}

#[get("/users/<user_id>/edit")]
pub fn edit(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let edit = match UserController::edit(user.clone(), user_id) {
        Ok(edit) => edit,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    let context = EditContext { user, edit };

    Ok(Template::render("user/edit", &context))
}

#[derive(FromForm)]
pub struct UpdatePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users/<user_id>", data = "<payload>")]
pub fn update(auth: Auth, user_id: i32, payload: Form<UpdatePayload>) -> impl Responder<'static> {
    let user = auth.clone().user();

    if !user.password_check(&payload.password) {
        return Err(Status::Forbidden);
    }

    match UserController::update(
        user,
        user_id,
        payload.name.clone(),
        payload.email.clone(),
        "guest".to_string(),
        payload.password.clone(),
    ) {
        Ok(user) => Ok(Redirect::to(format!("/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/users/<user_id>/delete")]
pub fn delete(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    match UserController::delete(user, user_id) {
        Ok(_) => Ok(Redirect::to("/users")),
        Err(e) => Err(Status::from(e)),
    }
}
