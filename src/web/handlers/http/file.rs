use controllers::{FileController, FolderController};
use db::models::{File, Folder, User};
use env::Env;
use rocket::data::Data;
use rocket::http::{ContentType, Status};
use rocket::request::Form;
use rocket::response::Stream;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use serde_derive::Serialize;
use std::fs;
use web::guards::auth::Auth;

#[get("/folders/<folder_id>/files")]
pub fn index(_auth: Auth, folder_id: i32) -> impl Responder<'static> {
    Redirect::to(format!("/folders/{}", folder_id))
}

#[derive(Serialize)]
pub struct ShowContext {
    user: User,
    folder: Folder,
    file: File,
}

#[get("/folders/<_folder_id>/files/<file_id>", rank = 2)]
pub fn show(auth: Auth, _folder_id: i32, file_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let file = match FileController::show(user.clone(), file_id) {
        Ok(file) => file,
        Err(e) => return Err(Status::from(e)),
    };

    let folder = match FolderController::show(user.clone(), file.folder_id()) {
        Ok(folder) => folder,
        Err(e) => return Err(Status::from(e)),
    };

    let context = ShowContext { user, folder, file };

    Ok(Template::render("file/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
    folder: Folder,
}

#[get("/folders/<folder_id>/files/create", rank = 1)]
pub fn create(auth: Auth, folder_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    if let Err(e) = FileController::create(user.clone()) {
        return Err(Status::from(e));
    }

    let folder = match FolderController::show(user.clone(), folder_id) {
        Ok(folder) => folder,
        Err(e) => return Err(Status::from(e)),
    };

    let context = CreateContext { user, folder };

    Ok(Template::render("file/create", &context))
}

#[post("/folders/<folder_id>/files", data = "<payload>")]
pub fn store(
    auth: Auth,
    folder_id: i32,
    payload: Data,
    content_type: &ContentType,
) -> impl Responder<'static> {
    let mut options = MultipartFormDataOptions::new();
    options
        .allowed_fields
        .push(MultipartFormDataField::file("file").size_limit(10000000000));

    let multipart_form_data = MultipartFormData::parse(content_type, payload, options).unwrap();

    let file = multipart_form_data.files.get("file");
    let mut name = String::new();

    let mut file = match file {
        Some(FileField::Single(file)) => {
            let res = match fs::File::open(file.path.clone()) {
                Ok(res) => res,
                Err(_) => return Err(Status::BadRequest),
            };

            if let Some(file_name) = &file.file_name {
                name = file_name.to_string();
            }

            res
        }
        _ => return Err(Status::BadRequest),
    };

    let user = auth.to_owned().user();

    let mut parts = name.splitn(2, ".");

    let stored = match FileController::store(
        user.clone(),
        parts.nth(0).unwrap_or("").to_string(),
        parts.nth(0).unwrap_or("").to_string(),
        user.id(),
        folder_id,
        false,
        &mut file,
    ) {
        Ok(file) => file,
        Err(e) => return Err(Status::from(e)),
    };
    Ok(Redirect::to(format!(
        "/folders/{}/files/{}",
        folder_id,
        stored.id()
    )))
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    folder: Folder,
    file: File,
}

#[get("/folders/<_folder_id>/files/<file_id>/edit")]
pub fn edit(auth: Auth, _folder_id: i32, file_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let file = match FileController::edit(user.clone(), file_id) {
        Ok(file) => file,
        Err(e) => return Err(Status::from(e)),
    };

    let folder = match FolderController::edit(user.clone(), file.folder_id()) {
        Ok(folder) => folder,
        Err(e) => return Err(Status::from(e)),
    };

    let context = EditContext { user, folder, file };

    Ok(Template::render("file/edit", &context))
}

#[derive(FromForm)]
pub struct UpdatePayload {
    name: String,
    extension: String,
    public: bool,
    folder_id: i32,
}

#[post("/folders/<_folder_id>/files/<file_id>", data = "<payload>")]
pub fn update(
    auth: Auth,
    _folder_id: i32,
    file_id: i32,
    payload: Form<UpdatePayload>,
) -> impl Responder<'static> {
    let user = auth.clone().user();

    match FileController::update(
        user.clone(),
        file_id,
        payload.name.to_owned(),
        payload.extension.to_owned(),
        payload.public,
        payload.folder_id,
    ) {
        Ok(file) => Ok(Redirect::to(format!(
            "/folders/{}/files/{}",
            file.folder_id(),
            file.id()
        ))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/folders/<_folder_id>/files/<file_id>/delete")]
pub fn delete(auth: Auth, _folder_id: i32, file_id: i32) -> impl Responder<'static> {
    let user = auth.user();

    match FileController::delete(user, file_id) {
        Ok(file) => Ok(Redirect::to(format!("/folders/{}", file.folder_id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[get("/folders/<_folder_id>/files/<file_id>/<_name>", rank = 1)]
pub fn download(
    auth: Auth,
    _folder_id: i32,
    file_id: i32,
    _name: String,
) -> impl Responder<'static> {
    let user = auth.clone().user();

    let stream = match FileController::contents(user, file_id) {
        Ok(stream) => stream,
        Err(e) => return Err(Status::from(e)),
    };

    let response = Stream::chunked(stream, Env::chunk_size());

    Ok(response)
}
