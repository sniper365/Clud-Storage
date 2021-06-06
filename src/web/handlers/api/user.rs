use controllers::UserController;
use db::presentation::ToJson;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{get, post};
use rocket_contrib::json::Json;
use serde_derive::Deserialize;
use web::guards::api::Auth;

#[get("/users")]
pub fn index(auth: Auth) -> impl Responder<'static> {
    let user = auth.clone().user();

    let users = match UserController::index(user.clone()) {
        Ok(users) => users,
        Err(e) => return Err(Status::from(e)),
    };

    Ok(users.to_json())
}

#[get("/users/<user_id>")]
pub fn show(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let show = match UserController::show(user.clone(), user_id) {
        Ok(user) => user,
        Err(e) => return Err(Status::from(e)),
    };

    Ok(show.to_json())
}

#[derive(Deserialize)]
pub struct StorePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users", data = "<payload>")]
pub fn store(auth: Auth, payload: Json<StorePayload>) -> impl Responder<'static> {
    let user = auth.clone().user();

    match UserController::store(
        user.clone(),
        payload.name.clone(),
        payload.email.clone(),
        "guest".to_string(),
        payload.password.clone(),
    ) {
        Ok(_) => {}
        Err(e) => return Err(Status::from(e)),
    };

    Ok(Status::Created)
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    name: String,
    email: String,
    password: String,
}

#[post("/users/<user_id>", data = "<payload>")]
pub fn update(auth: Auth, user_id: i32, payload: Json<UpdatePayload>) -> impl Responder<'static> {
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
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/users/<user_id>/delete")]
pub fn delete(auth: Auth, user_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    match UserController::delete(user, user_id) {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(Status::from(e)),
    }
}
