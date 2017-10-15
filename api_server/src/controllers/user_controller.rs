use rocket_contrib::Json;

use pg_pool::DbConn;

use models::user::User;

use requests::user_request;
use responses::user_response;

use guards::auth_guard::Auth;
use guards::admin_guard::Admin;

use rocket::http::Status;

use rocket::Response;

#[get("/users")]
fn index(conn: DbConn, _admin: Admin) -> Json<Vec<user_response::Show>> {
    let users = User::all(&conn).unwrap();

    let response: Vec<user_response::Show> = users.into_iter().map(| user | {
        user_response::Show {
            user_id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }).collect();

    return Json(response)
}

#[get("/users/<id>")]
fn show(conn: DbConn, auth: Auth, id: i32) -> Option<Json<user_response::Show>> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return None;
    }

    let user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return None,
    };

    return Some(Json(user_response::Show {
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        user_id: user.id,
    }))
}

#[post("/users", data="<request>")]
fn store(conn: DbConn, _admin: Admin, request: Json<user_request::Store>) -> Response<'static> {
    let new_user = User::new(request.0.first_name, request.0.last_name, request.0.email, request.0.password);

    let user = new_user.save(&conn).unwrap();

    let mut response = String::from("/api/users/");
    response.push(user.id as u8 as char);

    Response::build().status(Status::Created).finalize()
}

#[put("/users/<id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, id: i32, request: Json<user_request::Store>) -> Response<'static> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let mut user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::NotFound).finalize(),
    };

    user.first_name = request.0.first_name;
    user.last_name = request.0.last_name;
    user.email = request.0.email;

    if request.0.password.trim() != "" {
        user.set_password(request.0.password);
    }

    user.save(&conn).unwrap();

    Response::build().status(Status::NoContent).finalize()
}
