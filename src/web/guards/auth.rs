use auth::authenticate::Authenticate;
use auth::bearer::Token;
use auth::Auth as InternalAuth;
use db::models::User;
use rocket::http::Status;
use rocket::request;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use std::convert::TryFrom;

#[derive(Clone)]
pub struct Auth(User);

impl Auth {
    pub fn user(self) -> User {
        self.0
    }
}

#[derive(Debug)]
pub struct AuthError;

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let headers = request.headers();
        let cookies = request.cookies();

        let token: Token = match Token::try_from(cookies).or_else(|_| Token::try_from(headers)) {
            Ok(token) => token,
            Err(_) => return Outcome::Failure((Status::BadRequest, AuthError)),
        };

        match InternalAuth::Bearer(token).verify() {
            Ok(user) => Outcome::Success(Auth(user)),
            Err(_) => return Outcome::Failure((Status::Unauthorized, AuthError)),
        }
    }
}
