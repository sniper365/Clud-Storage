use rocket::Response;
use rocket::response::Failure;
use rocket::http::Status;
use std::fs::File;
use std::path::PathBuf;

use config;

#[get("/", rank = 4)]
pub fn index() -> Result<Response<'static>, Failure> {
    let file = match File::open(format!("{}/{}", config::build_dir(), config::app_index())) {
        Ok(file) => file,
        Err(_) => return Err(Failure(Status::NotFound)),
    };

    Ok(Response::build()
        .status(Status::Ok)
        .streamed_body(file)
        .finalize())
}

#[get("/api/<_path..>", rank = 2, format = "application/json")]
pub fn api_resource(_path: PathBuf) -> Result<Response<'static>, Failure> {
    Err(Failure(Status::NotFound))
}

#[get("/<path..>", rank = 3)]
pub fn resource(path: PathBuf) -> Result<Response<'static>, Failure> {
    let path = match path.to_str() {
        Some(path) => path,
        None => return Err(Failure(Status::NotFound)),
    };

    let file = match File::open(format!("{}/{path}", config::build_dir(), path = path)) {
        Ok(file) => file,
        Err(_) => {
            match File::open(format!("{}/{}", config::build_dir(), config::app_index())) {
                Ok(file) => file,
                Err(_) => return Err(Failure(Status::NotFound)),
            }
        },
    };

    Ok(Response::build()
        .status(Status::Ok)
        .streamed_body(file)
        .finalize())
}
