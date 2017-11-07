use rocket_contrib::Json;

use pg_pool::DbConn;

use models::user::{ User, Show };

use requests::user_request;

use guards::auth_guard::Auth;
use guards::admin_guard::Admin;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;

use serde_json;

#[get("/users")]
fn index(conn: DbConn, _admin: Admin) -> Response<'static> {
    let users = User::all(&conn).unwrap();

    let response: Vec<Show> = users.into_iter().map(| user | {
        user.into_show()
    }).collect();

    let response = serde_json::to_string(&response).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[get("/users/<id>")]
fn show(conn: DbConn, auth: Auth, id: i32) -> Response<'static> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let user = match User::find(id, &conn) {
        Ok(user) => user.into_show(),
        Err(_) => return Response::build().status(Status::NotFound).finalize(),
    };

    let response = serde_json::to_string(&user).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[post("/users", data="<request>")]
fn store(conn: DbConn, _admin: Admin, request: Json<user_request::Store>) -> Response<'static> {
    let new_user = User::new(request.0.name, request.0.email, request.0.password);

    let user = match new_user.save(&conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::SeeOther).finalize(),
    };

    let mut response = String::from("/api/users/");
    response.push(user.id as u8 as char);

    Response::build().status(Status::Created).finalize()
}

#[put("/users/<id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, id: i32, request: Json<user_request::Store>) -> Response<'static> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let mut user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::NotFound).finalize(),
    };

    user.name = request.0.name;
    user.email = request.0.email;

    if request.0.password.trim() != "" {
        user.set_password(request.0.password);
    }

    match user.save(&conn) {
        Ok(_) => Response::build().status(Status::NoContent).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}

#[delete("/users/<id>")]
fn delete(conn: DbConn, auth: Auth, id: i32) -> Response<'static> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::NotFound).finalize(),
    };

    match user.delete(&conn) {
        Ok(_) => Response::build().status(Status::Accepted).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}
