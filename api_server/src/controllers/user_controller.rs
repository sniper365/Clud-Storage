use rocket_contrib::Json;

use pg_pool::DbConn;

use models::user::User;
use models::folder::Folder;

use requests::user_request;

use guards::auth_guard::Auth;
use guards::admin_guard::Admin;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;

use serde_json;

use std::fs::DirBuilder;

use resources::AsResource;
use resources::user::User as UserResource;

#[get("/users")]
fn index(conn: DbConn, _admin: Admin) -> Response<'static> {
    let users = User::all(&conn).unwrap();

    let response: Vec<UserResource> = users.into_iter().map(| user | {
        user.as_resource()
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
        Ok(user) => user.as_resource(),
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
    let new_user = User::new(request.0.name, request.0.email, request.0.password, None);

    let mut user = match new_user.save(&conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::SeeOther).finalize(),
    };

    let root = match Folder::new(String::from("/"), None, user.id).save(&conn) {
        Ok( folder ) => { folder.id },
        Err(_) => return Response::build()
            .status(Status::InternalServerError)
            .finalize()
    };

    user.root = Some(root);

    match user.save(&conn) {
        Ok(_) => {},
        Err(_) => return Response::build()
            .status(Status::InternalServerError)
            .finalize()
    }

    let path = format!("storage/{user_id}", user_id = user.id);

    match DirBuilder::new().create(path) {
        Ok(_) => {},
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    Response::build()
        .status(Status::Created)
        .finalize()
}

#[put("/users/<id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, id: i32, request: Json<user_request::Update>) -> Response<'static> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let mut user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::NotFound).finalize(),
    };

    user.name = request.0.name;
    user.email = request.0.email;

    match user.save(&conn) {
        Ok(_) => Response::build().status(Status::NoContent).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}

#[put("/users/<id>/password", data="<request>")]
fn password(conn: DbConn, auth: Auth, id: i32, request: Json<user_request::Password>) -> Response<'static> {
    if auth.user.id != id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let mut user = match User::find(id, &conn) {
        Ok(user) => user,
        Err(_) => return Response::build().status(Status::NotFound).finalize(),
    };

    if request.0.password == request.0.password_confirmation {
        user.set_password(request.0.password);
    }
    else {
        return Response::build().status(Status::PreconditionFailed).finalize()
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
