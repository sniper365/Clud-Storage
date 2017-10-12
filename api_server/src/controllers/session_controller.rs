use rocket::http::Cookie;
use rocket::http::Cookies;
use rocket_contrib::Json;

use diesel;
use diesel::LoadDsl;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;

use bcrypt::verify;

use pg_pool::DbConn;

use models::user::User;

use guards::auth_guard::Token;

use requests::session_request;
use responses::session_response;

use std::str::FromStr;

#[post("/login", data="<request>")]
fn login(conn: DbConn, mut cookies: Cookies, request: Json<session_request::Login>) -> Json<session_response::Login> {
    use schema::users::dsl::{users, email as user_email };

    let mut user = match users.filter(user_email.eq(request.0.email))
        .first::<User>(&*conn) {
        Ok(user) => user,
        Err(_) => return Json(session_response::Login {
            success: false,
            user_id: None,
            token: None,
            message: String::from("Could not find a user with that email or password"),
        }),
    };

    let matched = match verify(&request.0.password, &user.password) {
        Ok(matched) => matched,
        Err(_) => return Json(session_response::Login {
            success: false,
            user_id: None,
            token: None,
            message: String::from("Password given does not match acceptance criteria"),
        }),
    };

    if matched {
        let token = Token::new(user.id).to_string();

        user.token = Some(String::from_str(&token).unwrap());

        user.save(&conn).unwrap();

        cookies.add_private(Cookie::new("session_token", token));

        return Json(session_response::Login {
            success: true,
            user_id: Some(user.id),
            token: Some(Token::new(user.id).to_string()),
            message: String::from("Successfully logged in"),
        });
    }

    return Json(session_response::Login {
        success: false,
        user_id: None,
        token: None,
        message: String::from("Could not find a user with that email or password"),
    });
}

#[get("/logout")]
fn logout(conn: DbConn, mut cookies: Cookies) -> Json<session_response::Logout> {
    use schema::users::dsl::{users, id, token as auth_token };

    let cookie = match cookies.get_private("session_token") {
        Some(cookie) => cookie,
        None => return Json(session_response::Logout {
            success: false,
            user_id: None,
            message: String::from("Attempted to logout without a session"),
        })
    };

    cookies.remove_private(Cookie::named("session_token"));

    let token = Token::from_json_string(cookie.value());

    diesel::update(users.filter(id.eq(token.user_id)))
        .set(auth_token.eq(""))
        .get_result::<User>(&*conn)
        .unwrap();

    Json(session_response::Logout {
        success: true,
        user_id: Some(token.user_id),
        message: String::from("Successfully destroyed session"),
    })
}
