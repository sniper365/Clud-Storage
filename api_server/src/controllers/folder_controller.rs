use pg_pool::DbConn;

use guards::auth_guard::Auth;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;

use serde_json;

use models::folder::{ Show, Folder };

use requests::folder_request;

use rocket_contrib::Json;

#[get("/users/<user_id>/folders")]
fn index(conn: DbConn, auth: Auth, user_id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
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

#[get("/users/<user_id>/root")]
fn root(conn: DbConn, auth: Auth, user_id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let folder = match auth.user.roots(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let found = match folder.first() {
        Some(found) => found.into_show(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let response = serde_json::to_string(&found).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[get("/users/<user_id>/folders/<id>")]
fn show(conn: DbConn, auth: Auth, user_id: i32, id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
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

#[get("/users/<user_id>/folders/<id>/children")]
fn children(conn: DbConn, auth: Auth, user_id: i32, id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
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

#[post("/users/<user_id>/folders", data="<request>")]
fn store(conn: DbConn, auth: Auth, user_id: i32, request: Json<folder_request::Store>) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let parent = match Folder::find(request.0.parent_id, &conn) {
        Ok(parent) => {
            if parent.user_id != auth.user.id {
                return Response::build().status(Status::Unauthorized).finalize();
            }

            parent
        },
        Err(_) => return Response::build().status(Status::BadRequest).finalize(),
    };

    match Folder::new(request.0.name, Some(parent.id), auth.user.id).save(&conn) {
        Ok(folder) => Response::build()
            .status(Status::Created)
            .sized_body(Cursor::new(serde_json::to_string(&folder.into_show()).unwrap()))
            .finalize(),
        Err(_) => Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .finalize()
    }
}

#[put("/users/<user_id>/folders/<folder_id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, request: Json<folder_request::Store>) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let mut folder = match Folder::find(folder_id, &conn) {
        Ok(folder) => {
            if folder.user_id != auth.user.id {
                return Response::build().status(Status::Unauthorized).finalize();
            }

            folder
        },
        Err(_) => return Response::build().status(Status::BadRequest).finalize(),
    };

    if folder.parent_id.is_none() {
        return Response::build().status(Status::Forbidden).finalize()
    }

    folder.name = request.0.name;
    folder.parent_id = Some(request.0.parent_id);

    match folder.save(&conn) {
        Ok(_) => Response::build().status(Status::NoContent).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}

#[delete("/users/<user_id>/folders/<folder_id>")]
fn delete(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
        return Response::build().status(Status::NotFound).finalize();
    }

    let folder = match Folder::find(folder_id, &conn) {
        Ok(folder) => {
            if folder.user_id != auth.user.id {
                return Response::build().status(Status::Unauthorized).finalize();
            }

            folder
        },
        Err(_) => return Response::build().status(Status::BadRequest).finalize(),
    };

    if folder.parent_id.is_none() {
        return Response::build().status(Status::Forbidden).finalize()
    }

    match folder.delete(&conn) {
        Ok(_) => Response::build().status(Status::Accepted).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}
