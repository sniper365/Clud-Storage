use ::chrono::prelude::*;
use ::serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub user_id: i32,
    pub timestamp: String,
}

impl Token {
    pub fn new(user_id: i32) -> Token {
        Token {
            user_id: user_id,
            timestamp: Utc::now().format("%+").to_string(),
        }
    }

    pub fn from_json_string(json: &str) -> Token {
        serde_json::from_str(json).unwrap()
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

use rocket::request::FromRequest;
use rocket::{Request, State, Outcome};
use rocket::request;
use rocket::http::Status;

use models::user::User;

use diesel::prelude::*;

pub struct Auth {
    user: User,
}

use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

type ManagedPgConn = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ManagedPgConn>;

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        let pool = request.guard::<State<Pool>>()?;

        match pool.get() {
            Ok(conn) => {
                use schema::users::dsl::{ users, token as auth_token };

                let mut cookies = request.cookies();

                match cookies.get_private("session_token") {
                    Some(session_token) => {
                        let token = session_token.value();

                        match users.filter(auth_token.eq(token)).first::<User>(&*conn) {
                            Ok(user) => {
                                Outcome::Success(Auth { user: user })
                            },
                            Err(_) => {
                                Outcome::Failure((Status::Unauthorized, ()))
                            }
                        }
                    },
                    None => {
                        Outcome::Failure((Status::Unauthorized, ()))
                    },
                }
            },
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
