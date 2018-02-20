use pg_pool::DbConn;

use guards::auth_guard::Auth;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;
use rocket::response::Failure;

use serde_json;

use models::folder::Folder;

use requests::folder_request;

use rocket_contrib::Json;

use resources::AsResource;
use resources::folder::Folder as FolderResource;

#[get("/users/<user_id>/folders")]
fn index(conn: DbConn, auth: Auth, user_id: i32) -> Result<Response<'static>, Failure> {
    if auth.user.id != user_id {
        return Err(Failure(Status::Unauthorized));
    }

    let folders: Vec<FolderResource> = match auth.user.folders(&conn) {
        Ok(folders) => folders.into_iter().map( | folder | {
            folder.as_resource()
        }).collect(),
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let response = serde_json::to_string(&folders).unwrap();

    Ok(Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize())
}

#[get("/users/<user_id>/folders/<id>")]
fn show(conn: DbConn, auth: Auth, user_id: i32, id: i32) -> Result<Response<'static>, Failure> {
    if auth.user.id != user_id {
        return Err(Failure(Status::Unauthorized));
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let found = match folders.iter().position( | folder | folder.id == id ) {
        Some(found) => folders[found].as_resource(),
        None => return Err(Failure(Status::NotFound)),
    };

    let response = serde_json::to_string(&found).unwrap();

    Ok(Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize())
}

#[get("/users/<user_id>/folders/<id>/children")]
fn children(conn: DbConn, auth: Auth, user_id: i32, id: i32) -> Result<Response<'static>, Failure> {
    if auth.user.id != user_id {
        return Err(Failure(Status::Unauthorized));
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let found = match folders.iter().position( | folder | folder.id == id ) {
        Some(found) => match folders[found].children(&conn) {
            Ok(children) => children,
            Err(_) => return Err(Failure(Status::InternalServerError)),
        },
        None => return Err(Failure(Status::NotFound)),
    };

    let response: Vec<FolderResource> = found.into_iter().map( | folder | {
        folder.as_resource()
    }).collect();

    let response = serde_json::to_string(&response).unwrap();

    Ok(Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize())
}

#[post("/users/<user_id>/folders", data="<request>")]
fn store(conn: DbConn, auth: Auth, user_id: i32, request: Json<folder_request::Store>) -> Result<Response<'static>, Failure> {
    if auth.user.id != user_id {
        return Err(Failure(Status::Unauthorized));
    }

    let parent = match Folder::find(request.0.parent_id, &conn) {
        Ok(parent) => {
            if parent.user_id != auth.user.id {
                return Err(Failure(Status::Unauthorized));
            }

            parent
        },
        Err(_) => return Err(Failure(Status::BadRequest)),
    };

    match Folder::new(request.0.name, Some(parent.id), auth.user.id).save(&conn) {
        Ok(folder) => Ok(Response::build()
            .status(Status::Created)
            .sized_body(Cursor::new(serde_json::to_string(&folder.as_resource()).unwrap()))
            .finalize()),
        Err(_) => Err(Failure(Status::InternalServerError))
    }
}

#[put("/users/<user_id>/folders/<folder_id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, request: Json<folder_request::Store>) -> Result<Response<'static>, Failure> {
    if auth.user.id != user_id {
        return Err(Failure(Status::Unauthorized));
    }

    let mut folder = match Folder::find(folder_id, &conn) {
        Ok(folder) => {
            if folder.user_id != auth.user.id {
                return Err(Failure(Status::Unauthorized));
            }

            folder
        },
        Err(_) => return Err(Failure(Status::BadRequest)),
    };

    if folder.parent_id.is_none() {
        return Err(Failure(Status::Forbidden))
    }

    folder.name = request.0.name;
    folder.parent_id = Some(request.0.parent_id);

    match folder.save(&conn) {
        Ok(_) => Ok(Response::build().status(Status::NoContent).finalize()),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}

#[delete("/users/<user_id>/folders/<folder_id>")]
fn delete(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32) -> Result<Response<'static>, Failure> {
    if auth.user.id != user_id {
        return Err(Failure(Status::NotFound));
    }

    let folder = match Folder::find(folder_id, &conn) {
        Ok(folder) => {
            if folder.user_id != auth.user.id {
                return Err(Failure(Status::Unauthorized));
            }

            folder
        },
        Err(_) => return Err(Failure(Status::BadRequest)),
    };

    if folder.parent_id.is_none() {
        return Err(Failure(Status::Forbidden))
    }

    match folder.delete(&conn) {
        Ok(_) => Ok(Response::build().status(Status::Accepted).finalize()),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}
