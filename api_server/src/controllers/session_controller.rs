use rocket_contrib::Json;

use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;

use bcrypt::verify;

use pg_pool::DbConn;

use models::user::User;

use requests::session_request;
use responses::session_response;

use libraries::jwt::Token;

#[post("/login", data="<request>")]
fn login(conn: DbConn, request: Json<session_request::Login>) -> Json<session_response::Login> {
    use schema::users::dsl::{users, email as user_email };

    // Find the user in the database that they claim to be
    let user = match users.filter(user_email.eq(request.0.email))
        .first::<User>(&*conn) {
        Ok(user) => user,
        Err(_) => return Json(session_response::Login {
            success: false,
            user_id: None,
            token: None,
            message: "Could not find a user with that email or password".to_string(),
        }),
    };

    // Check their password: if it fails to encrypt: their password is unacceptable
    let matched = match verify(&request.0.password, &user.password) {
        Ok(matched) => matched,
        Err(_) => return Json(session_response::Login {
            success: false,
            user_id: None,
            token: None,
            message: "Password given does not match acceptance criteria".to_string(),
        }),
    };

    // If it matched: make a token of their identity and return it
    if matched {
        let token = Token::new(user.id, user.display_name()).encode();

        return Json(session_response::Login {
            success: true,
            user_id: Some(user.id),
            token: Some(token),
            message: "Successfully logged in".to_string(),
        })
    }

    // If it failed, return a failure
    return Json(session_response::Login {
        success: false,
        user_id: None,
        token: None,
        message: "Could not find a user with that email or password".to_string(),
    });
}
