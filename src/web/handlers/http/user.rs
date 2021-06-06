use crate::controllers::user::UpdateRequest;
use crate::controllers::user::StoreRequest;
use crate::entities::models::User;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use crate::web::guards::auth::Auth;
use crate::web::state::State;
use crate::controllers::UserController;

#[derive(Serialize)]
pub struct IndexContext {
    user: User,
    users: Vec<User>,
}

#[get("/users")]
pub fn index(auth: Auth, state: State) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let users = match user_controller.index(user.clone()) {
        Ok(users) => users,
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    let context = state.into_context(IndexContext { user, users });

    Ok(Template::render("user/index", &context))
}

#[derive(Serialize)]
pub struct ShowContext {
    user: User,
    show: User,
}

#[get("/users/<user_id>")]
pub fn show(auth: Auth, state: State, user_id: i32) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let show = match user_controller.show(user.clone(), user_id) {
        Ok(user) => user,
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    let context = state.into_context(ShowContext { user, show });

    Ok(Template::render("user/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
}

#[get("/users/create")]
pub fn create(auth: Auth, state: State) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    if let Err(e) = user_controller.create(user.clone()) {
        return Err(Status::from(e));
    }

    let context = state.into_context(CreateContext { user });

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
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let store_request = StoreRequest {
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: "guest".to_string(),
        password: payload.password.clone()
    };

    let user = match user_controller.store(user.clone(), store_request) {
        Ok(user) => user,
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    Ok(Redirect::to(format!("/users/{}", user.id())))
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    edit: User,
}

#[get("/users/<user_id>/edit")]
pub fn edit(auth: Auth, state: State, user_id: i32) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    let edit = match user_controller.edit(user.clone(), user_id) {
        Ok(edit) => edit,
        Err(e) => {
            log!(
                e.level(),
                "Request from user \"{}\" returned \"{}\"",
                user.id(),
                e
            );
            return Err(Status::from(e));
        }
    };

    let context = state.into_context(EditContext { user, edit });

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
    let user_controller = resolve!(UserController);
    let user = auth.user();

    if !user.password_check(&payload.password) {
        return Err(Status::Forbidden);
    }

    let update_request = UpdateRequest {
        user_id,
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: user.role().to_string()
    };

    match user_controller.update(user, update_request) {
        Ok(user) => Ok(Redirect::to(format!("/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/users/<user_id>/delete")]
pub fn delete(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    match user_controller.delete(user, user_id) {
        Ok(_) => Ok(Redirect::to("/users")),
        Err(e) => Err(Status::from(e)),
    }
}

#[derive(FromForm)]
pub struct UpdatePasswordPayload {
    old_password: String,
    password: String,
}

#[post("/users/<user_id>/password", data = "<payload>")]
pub fn update_password(
    auth: Auth,
    user_id: i32,
    payload: Form<UpdatePasswordPayload>,
) -> impl Responder<'static> {
    let user_controller = resolve!(UserController);
    let user = auth.user();

    if !user.password_check(&payload.old_password) {
        return Err(Status::Forbidden);
    }

    match user_controller.update_password(user, user_id, payload.password.to_string()) {
        Ok(user) => Ok(Redirect::to(format!("/users/{}", user.id()))),
        Err(e) => Err(Status::from(e)),
    }
}
