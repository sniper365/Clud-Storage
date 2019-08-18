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
        let headers = request.headers();
        let mut cookies = request.cookies();

        let token: Token;

        if let Some(cookie) = cookies.get_private("token") {
            token = Token::new(cookie.value().to_string());
        } else {
            let mut auth_header = match headers.get_one("Authorization") {
                // We want it back in the format <TYPE auth>
                Some(token) => token.split_whitespace(),
                None => return Outcome::Failure((Status::Unauthorized, AuthError)),
            };

            let auth_value = match auth_header.next() {
                Some("Bearer") => match auth_header.next() {
                    Some(token) => token,
                    None => return Outcome::Failure((Status::BadRequest, AuthError)),
                },
                _ => return Outcome::Failure((Status::BadRequest, AuthError)),
            };

            token = Token::new(auth_value.to_string());
        }

        match InternalAuth::Bearer(token).verify() {
            Ok(user) => Outcome::Success(Auth(user)),
            Err(_) => return Outcome::Failure((Status::Unauthorized, AuthError)),
        }
    }
}
