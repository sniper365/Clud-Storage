use controllers::FileController;
use db::builders::{Builder, UserBuilder};
use db::models::{File, User};
use rocket::get;
use rocket::http::Status;
use rocket::response::{Body, Responder, Response};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use std::io::Cursor;

#[derive(Serialize)]
pub struct FileContext {
    user: User,
    file: File,
}

#[get("/public/<file_id>")]
pub fn file(file_id: i32) -> impl Responder<'static> {
    let user = UserBuilder::new()
        .with_name("Guest".to_string())
        .with_role("guest".to_string())
        .build();

    let file = match FileController::show(user.clone(), file_id) {
        Ok(file) => file,
        Err(e) => return Err(Status::from(e)),
    };

    let context = FileContext { user, file };

    Ok(Template::render("public/file", &context))
}

#[get("/public/<file_id>/download")]
pub fn download(file_id: i32) -> impl Responder<'static> {
    let user = UserBuilder::new()
        .with_name("Guest".to_string())
        .with_role("guest".to_string())
        .build();

    let contents = match FileController::contents(user, file_id) {
        Ok(contents) => contents,
        Err(e) => return Err(Status::from(e)),
    };

    Ok(Response::build()
        .status(Status::Ok)
        .raw_body(Body::Sized(
            Cursor::new(contents.clone()),
            contents.len() as u64,
        ))
        .finalize())
}
