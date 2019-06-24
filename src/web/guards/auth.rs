use auth::authenticate::Authenticate;
use auth::bearer::Token;
use auth::Auth as InternalAuth;
use db::models::User;
use rocket::http::Status;
use rocket::request;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;

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
        let mut cookies = request.cookies();

        let token = match cookies.get_private("token") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, AuthError)),
        };

        let token = Token::new(token.value().to_string());

        match InternalAuth::Bearer(token).verify() {
            Ok(user) => Outcome::Success(Auth(user)),
            Err(_) => return Outcome::Failure((Status::Unauthorized, AuthError)),
        }
    }
}
