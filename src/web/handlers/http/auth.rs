use auth::authenticate::Authenticate;
use auth::basic::Credentials;
use auth::bearer::Token;
use auth::Auth;
use db::models::User;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use std::convert::TryFrom;

#[derive(Serialize)]
struct LoginContext {}

#[get("/login")]
pub fn login() -> impl Responder<'static> {
    let context = LoginContext {};

    Template::render("auth/login", &context)
}

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data = "<payload>")]
pub fn authenticate(mut cookies: Cookies, payload: Form<LoginForm>) -> impl Responder<'static> {
    let credentials = Credentials::new(payload.email.clone(), payload.password.clone());

    let user: User = match Auth::Basic(credentials).verify() {
        Ok(user) => user,
        Err(_) => return Ok(Redirect::to("/login")),
    };

    let token = match Token::try_from(user.clone()) {
        Ok(token) => token,
        Err(e) => {
            log!("error", "500 Internal Server Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    cookies.add_private(Cookie::new("token", token.to_string()));

    log!("debug", "Got session from user {}", user.id());

    Ok(Redirect::to("/"))
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> impl Responder<'static> {
    cookies.remove_private(Cookie::named("token"));

    Redirect::to("/login")
}
