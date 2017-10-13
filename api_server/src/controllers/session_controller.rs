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

use guards::auth_guard::Auth;

#[post("/login", data="<request>")]
fn login(conn: DbConn, mut cookies: Cookies, request: Json<session_request::Login>) -> Json<session_response::Login> {
    use schema::users::dsl::{users, email as user_email };

    // let new_user = User::new("Charles".to_string(), "Bassett".to_string(), "chasb96@gmail.com".to_string(), "password".to_string());
    //
    // let user = new_user.save(&conn).unwrap();

    let mut user = match users.filter(user_email.eq(request.0.email))
        .first::<User>(&*conn) {
        Ok(user) => user,
        Err(_) => return Json(session_response::Login {
            success: false,
            user_id: None,
            token: None,
            message: "Could not find a user with that email or password".to_string(),
        }),
    };

    let matched = match verify(&request.0.password, &user.password) {
        Ok(matched) => matched,
        Err(_) => return Json(session_response::Login {
            success: false,
            user_id: None,
            token: None,
            message: "Password given does not match acceptance criteria".to_string(),
        }),
    };

    if matched {
        let token = Token::new(user.id).to_string();

        cookies.add(Cookie::build("session_token", token).path("/").finish());
        let token = cookies.get("session_token").unwrap().value().to_string();

        user.token = Some(String::from_str(&token).unwrap());

        user.save(&conn).unwrap();

        return Json(session_response::Login {
            success: true,
            user_id: Some(user.id),
            token: Some(token),
            message: "Successfully logged in".to_string(),
        });
    }

    return Json(session_response::Login {
        success: false,
        user_id: None,
        token: None,
        message: "Could not find a user with that email or password".to_string(),
    });
}

#[get("/logout")]
fn logout(conn: DbConn,  auth: Auth) -> Json<session_response::Logout> {
    use schema::users::dsl::{users, id, token as auth_token };

    diesel::update(users.filter(id.eq(auth.user.id)))
        .set(auth_token.eq(""))
        .get_result::<User>(&*conn)
        .unwrap();

    Json(session_response::Logout {
        success: true,
        user_id: Some(auth.user.id),
        message: "Successfully destroyed session".to_string(),
    })
}
