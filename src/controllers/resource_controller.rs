use rocket::Response;
use rocket::response::Failure;
use rocket::http::Status;
use std::fs::File;
use std::path::PathBuf;

#[get("/", rank = 2)]
pub fn index() -> Result<Response<'static>, Failure> {
    let file = match File::open(format!("frontend/build/index.html")) {
        Ok(file) => file,
        Err(_) => return Err(Failure(Status::NotFound)),
    };

    Ok(Response::build()
    .status(Status::Ok)
    .streamed_body(file)
    .finalize())
}

#[get("/<path..>", rank = 2)]
pub fn resource(path: PathBuf) -> Result<Response<'static>, Failure> {
    let path = match path.to_str() {
        Some(path) => path,
        None => return Err(Failure(Status::NotFound)),
    };

    let file = match File::open(format!("frontend/build/{path}", path = path)) {
        Ok(file) => file,
        Err(_) => return Err(Failure(Status::NotFound)),
    };

    Ok(Response::build()
        .status(Status::Ok)
        .streamed_body(file)
        .finalize())
}
