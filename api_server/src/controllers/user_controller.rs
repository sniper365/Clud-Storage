use rocket_contrib::Json;

use pg_pool::DbConn;

use models::user::User;

use requests::user_request;
use responses::user_response;

use guards::auth_guard::Auth;

use rocket::response::status::*;
use rocket::http::Status;

#[get("/users")]
fn index(conn: DbConn, auth: Auth) -> Result<Json<Vec<user_response::Show>>, Custom<Status>> {
    if !auth.user.is_admin(&conn) {
        return Err(Custom(Status::Unauthorized, Status::Unauthorized))
    }

    let users = match User::all(&conn) {
        Ok(users) => users,
        Err(_) => return Err(Custom(Status::InternalServerError, Status::InternalServerError))
    };

    let response: Vec<user_response::Show> = users.into_iter().map(| user | {
        user_response::Show {
            user_id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }).collect();

    return Ok(Json(response))
}

#[get("/users/<id>")]
fn show(conn: DbConn, auth: Auth, id: i32) -> Result<Json<user_response::Show>, Custom<Status>> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Err(Custom(Status::Unauthorized, Status::Unauthorized));
    }

    let user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Custom(Status::NotFound, Status::NotFound)),
    };

    return Ok(Json(user_response::Show {
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        user_id: user.id,
    }))
}

#[post("/users", data="<request>")]
fn store(conn: DbConn, auth: Auth, request: Json<user_request::Store>) -> Result<Created<String>, Custom<Status>> {
    if !auth.user.is_admin(&conn) {
        return Err(Custom(Status::Unauthorized, Status::Unauthorized));
    }

    let new_user = User::new(request.0.first_name, request.0.last_name, request.0.email, request.0.password);

    let user = new_user.save(&conn).unwrap();

    let mut response = String::from("/api/users/");
    response.push(user.id as u8 as char);

    Ok(Created(response, Some("Successfully created user".to_string())))
}

#[put("/users/<id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, id: i32, request: Json<user_request::Store>) -> Result<NoContent, Custom<Status>> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Err(Custom(Status::Unauthorized, Status::Unauthorized));
    }

    let mut user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Custom(Status::NotFound, Status::NotFound)),
    };

    user.first_name = request.0.first_name;
    user.last_name = request.0.last_name;
    user.email = request.0.email;

    if request.0.password.trim() != "" {
        user.set_password(request.0.password);
    }

    user.save(&conn).unwrap();

    Ok(NoContent)
}
