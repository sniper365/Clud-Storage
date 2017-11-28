use pg_pool::DbConn;

use guards::auth_guard::Auth;

use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use rocket::Response;
use rocket::Data;
use std::path::Path;

use serde_json;

use models::file::{ Show, File };

use requests::file_request;

use rocket_contrib::Json;

use time;
use rand;
use rand::Rng;

#[get("/users/<user_id>/folders/<folder_id>/files")]
fn index(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
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

    let response: Vec<Show> = files.into_iter().map( | file | {
        file.into_show()
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
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
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
        Some(found) => files[found].into_show(),
        None => return Response::build().status(Status::NotFound).finalize(),
    };

    let response = serde_json::to_string(&found).unwrap();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(response))
        .finalize()
}


#[post("/users/<user_id>/folders/<_folder_id>/files", format="text/plain", data="<file>")]
fn store_file(conn: DbConn, auth: Auth, user_id: i32, _folder_id: i32, file: Data) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
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
            .header(ContentType::JSON)
            .sized_body(Cursor::new(file_name))
            .finalize(),
        Err(e) => Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(e.to_string()))
            .finalize(),
    }
}

#[post("/users/<user_id>/folders/<folder_id>/files", format="application/json", data="<request>")]
fn store(conn: DbConn, auth: Auth, user_id: i32, folder_id: i32, request: Json<file_request::Store>) -> Response<'static> {
    if auth.user.id != user_id && !auth.user.is_admin(&conn) {
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
        Ok(_) => Response::build()
            .status(Status::Created)
            .header(ContentType::JSON)
            .finalize(),
        Err(_) => Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .finalize()
    }
}
