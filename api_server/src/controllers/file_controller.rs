use pg_pool::DbConn;

use guards::auth_guard::Auth;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;
use rocket::Data;
use std::path::Path;

use serde_json;

use models::file::File;

use requests::file_request;

use rocket_contrib::Json;

use resources::AsResource;
use resources::file::File as FileResource;

use time;
use rand;
use rand::Rng;
use std::fs;

#[get("/users/<user_id>/folders/<folder_id>/files")]
fn index(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let found = match folders.iter().position( | folder | folder.id == folder_id ) {
        // We already know the index, we already know one exists there.
        Some(found) => folders.get(found).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let files = match found.files(&conn) {
        Ok(files) => files,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let response: Vec<FileResource> = files.into_iter().map( | file | {
        file.as_resource()
    }).collect();

    let response = serde_json::to_string(&response).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[get("/users/<user_id>/folders/<folder_id>/files/<file_id>")]
fn show(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, file_id: i32) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let folder = match folders.iter().position( | folder | folder.id == folder_id ) {
        // We already know the index, we already know one exists there.
        Some(folder) => folders.get(folder).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let files = match folder.files(&conn) {
        Ok(files) => files,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let found = match files.iter().position( | file | file.id == file_id ) {
        // We already know the index, we already know one exists there.
        Some(found) => files[found].as_resource(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let response = serde_json::to_string(&found).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}

#[get("/users/<user_id>/folders/<folder_id>/files/<file_id>/download")]
fn download(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, file_id: i32) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let folder = match folders.iter().position( | folder | folder.id == folder_id ) {
        // We already know the index, we already know one exists there.
        Some(folder) => folders.get(folder).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let files = match folder.files(&conn) {
        Ok(files) => files,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let found = match files.iter().position( | file | file.id == file_id ) {
        // We already know the index, we already know one exists there.
        Some(found) => files.get(found).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let path = format!("storage/{user_id}/{file_name}", user_id = user_id, file_name = &found.file_name);

    Response::build()
        .status(Status::Ok)
        .streamed_body(fs::File::open(path).unwrap())
        .finalize()
}


#[post("/users/<user_id>/folders/<_folder_id>/files", format="text/plain", data="<file>")]
fn store_file(auth: Auth, user_id: i32, _folder_id: i32, file: Data) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let timestamp = time::now();

    let random_bytes: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(16)
        .collect();

    let file_name = format!("{timestamp}{random_bytes}", timestamp = timestamp.rfc3339(), random_bytes = random_bytes);

    let path = format!("storage/{user_id}/{file_name}", user_id = user_id, file_name = &file_name);

    match file.stream_to_file(Path::new(&path)) {
        Ok(_) => Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(file_name))
            .finalize(),
        Err(_) => Response::build()
            .status(Status::InternalServerError)
            .finalize(),
    }
}

#[post("/users/<user_id>/folders/<folder_id>/files", format="application/json", data="<request>")]
fn store(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, request: Json<file_request::Store>) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let path = format!("storage/{user_id}/{file_name}", user_id = user_id, file_name = request.0.file_name);

    if !Path::new(&path).exists() {
        return Response::build()
            .status(Status::NotFound)
            .header(ContentType::JSON)
            .finalize();
    }

    match File::new(request.0.name, request.0.file_name, folder_id, request.0.extension).save(&conn) {
        Ok(file) => Response::build()
            .status(Status::Created)
            .sized_body(Cursor::new(serde_json::to_string(&file.as_resource()).unwrap()))
            .finalize(),
        Err(_) => Response::build()
            .status(Status::InternalServerError)
            .finalize()
    }
}

#[put("/users/<user_id>/folders/<folder_id>/files/<file_id>", data="<request>")]
fn update(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, file_id: i32, request: Json<file_request::Store>) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::Unauthorized).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let folder = match folders.iter().position( | folder | folder.id == folder_id ) {
        // We already know the index, we already know one exists there.
        Some(folder) => folders.get(folder).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let mut files = match folder.files(&conn) {
        Ok(files) => files,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let file = match files.iter().position( | file | file.id == file_id ) {
        // We already know the index, we already know one exists there.
        Some(file) => files.get_mut(file).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    file.name = request.0.name;
    file.extension = request.0.extension;

    match file.save(&conn) {
        Ok(_) => Response::build().status(Status::NoContent).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}

#[delete("/users/<user_id>/folders/<folder_id>/files/<file_id>")]
fn delete(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, file_id: i32) -> Response<'static> {
    if auth.user.id != user_id {
        return Response::build().status(Status::NotFound).finalize();
    }

    let folders = match auth.user.folders(&conn) {
        Ok(folders) => folders,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let folder = match folders.iter().position( | folder | folder.id == folder_id ) {
        // We already know the index, we already know one exists there.
        Some(folder) => folders.get(folder).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let files = match folder.files(&conn) {
        Ok(files) => files,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize(),
    };

    let file = match files.iter().position( | file | file.id == file_id ) {
        // We already know the index, we already know one exists there.
        Some(file) => files.get(file).unwrap(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    match file.delete(&conn) {
        Ok(_) => Response::build().status(Status::Accepted).finalize(),
        Err(_) => Response::build().status(Status::InternalServerError).finalize(),
    }
}
