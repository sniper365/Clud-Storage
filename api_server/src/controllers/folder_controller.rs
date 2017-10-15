use pg_pool::DbConn;

use guards::auth_guard::Auth;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;

use serde_json;

use models::folder::Show;

#[get("/users/<user_id>/folders")]
fn index(conn: DbConn, auth: Auth, user_id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let response: Vec<Show> = folders.into_iter().map( | folder | {
        folder.into_show()
    }).collect();

    let response = serde_json::to_string(&response).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[get("/users/<user_id>/folders/<id>")]
fn show(conn: DbConn, auth: Auth, user_id: i32, id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let found = match folders.iter().position( | folder | folder.id == id ) {
        Some(found) => folders[found].into_show(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let response = serde_json::to_string(&found).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[get("/users/<user_id>/folders/<id>/sub-directories")]
fn children(conn: DbConn, auth: Auth, user_id: i32, id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let found = match folders.iter().position( | folder | folder.id == id ) {
        Some(found) => match folders[found].children(&conn) {
            Ok(children) => children,
            Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
        },
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let response: Vec<Show> = found.into_iter().map( | folder | {
        folder.into_show()
    }).collect();

    let response = serde_json::to_string(&response).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}
