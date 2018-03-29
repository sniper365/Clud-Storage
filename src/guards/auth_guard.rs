use libraries::jwt::Token;

use rocket::request::FromRequest;
use rocket::{Request, State, Outcome};
use rocket::request;
use rocket::http::Status;

use models::user::User;

use diesel::prelude::*;

pub struct Auth {
    pub user: User,
}

use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use diesel::FirstDsl;

type ManagedPgConn = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ManagedPgConn>;

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        // I want access to the database pool and the users table
        //  for lookup if the token is a success.
        let pool = request.guard::<State<Pool>>()?;
        use schema::users::dsl::users;

        // Get a connection; if one is not available, reply as such
        let conn = match pool.get() {
            Ok(conn) => conn,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        // Get the Authorization header.
        // As mentioned below: I only currently want to impliment one type of authorization
        // So if 10 are sent, or none are sent, I only care about 1: and I want Bearer
        let authorization = match request.headers().get("Authorization").nth(0) {
            Some(authorization) => authorization,
            None => return Outcome::Failure((Status::BadRequest, ())),
        };

        // I don't at this moment particularly care about Basic Auth
        // I am only accepting Bearer Tokens.
        // If Authorization Basic is sent, then this will fail as intended anyways
        match authorization.split(" ").collect::<Vec<&str>>().get(1) {
            Some(bearer_token) => {
                // Try to build and authenticate the token from this:
                // If the token is malformed or doesn't match, then they're not who they say they are
                // From `frankie_jwt`: in the libraries/jwt we wrap the decode function into a helper on the object
                // Because of that: we can get the token and authenticate in one hit
                let (_header, payload) = match Token::from_encoded( bearer_token.to_string() ) {
                    Ok(token) => token,
                    Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
                };

                // Now grabbing the user_id from the token: all the way up to here,
                // we've already done all the steps to validate everything is correct
                // and we've already authenticated them:
                // Therefore: we know that this user_id is valid.
                // For them to manage to get an incorrect user_id and match the signature
                // is incredibly small: so here, we know it won't fail, only the compiler doesn't.
                // Because of this; we're going to replace everything with zeroes for our error case
                let user_id = payload.get("user_id").map(| user | {
                    user.parse::<i32>().unwrap_or( 0 )
                }).unwrap_or( 0 );

                // Give Outcome success here; as mentioned above: the secondary case is
                // statistically impossible, we're keeping it there to keep the compiler happy
                match users.find(user_id).first::<User>(&*conn) {
                    Ok(user) => return Outcome::Success(Auth { user: user }),
                    Err(_) => return Outcome::Failure((Status::Unauthorized, ()))
                }
            },
            None => return Outcome::Failure((Status::BadRequest, ())),
        };
    }
}
