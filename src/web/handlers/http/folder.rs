use controllers::{FileController, FolderController};
use db::models::{File, Folder, User};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Redirect, Responder};
use rocket::FromForm;
use rocket::{get, post};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::auth::Auth;

#[get("/folders")]
pub fn index(_auth: Auth) -> impl Responder<'static> {
    Redirect::to("/")
}

#[derive(Serialize)]
struct FolderContext {
    user: User,
    folder: Folder,
    folders: Vec<Folder>,
    files: Vec<File>,
}

#[get("/folders/<folder_id>")]
pub fn show(auth: Auth, folder_id: i32) -> impl Responder<'static> {
    let user = auth.to_owned().user();

    let folder = match FolderController::show(user.clone(), folder_id) {
        Ok(folder) => folder,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    let subfolders = match FolderController::index(user.clone(), Some(folder_id)) {
        Ok(subfolders) => subfolders,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    let files = match FileController::index(user.clone(), folder.id()) {
        Ok(files) => files,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    let context = FolderContext {
        user,
        folder,
        folders: subfolders,
        files,
    };

    Ok(Template::render("folder/show", &context))
}

#[derive(Serialize)]
pub struct CreateContext {
    user: User,
    parent: Option<Folder>,
}

#[get("/folders/create?<folder_id>")]
pub fn create(auth: Auth, folder_id: Option<i32>) -> impl Responder<'static> {
    let user = auth.clone().user();

    let mut parent = None;
    if let Some(folder_id) = folder_id {
        parent = match FolderController::show(user.clone(), folder_id) {
            Ok(parent) => Some(parent),
            Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
        };
    }

    let context = CreateContext {
        user: user.clone(),
        parent,
    };

    match FolderController::create(user.clone()) {
        Ok(_) => Ok(Template::render("folder/create", &context)),
        Err(e) => Err(Status::from(e)),
    }
}

#[derive(FromForm)]
pub struct StorePayload {
    name: String,
}

#[post("/folders?<folder_id>", data = "<payload>")]
pub fn store(
    auth: Auth,
    folder_id: Option<i32>,
    payload: Form<StorePayload>,
) -> impl Responder<'static> {
    let user = auth.clone().user();

    match FolderController::store(user.clone(), payload.name.to_owned(), user.id(), folder_id) {
        Ok(folder) => Ok(Redirect::to(format!("/folders/{}", folder.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[derive(Serialize)]
pub struct EditContext {
    user: User,
    parent: Option<Folder>,
    folder: Folder,
}

#[get("/folders/<folder_id>/edit")]
pub fn edit(auth: Auth, folder_id: i32) -> impl Responder<'static> {
    let user = auth.clone().user();

    let mut parent = None;
    let folder = match FolderController::edit(user.clone(), folder_id) {
        Ok(folder) => folder,
        Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
    };

    if let Some(parent_id) = folder.parent_id() {
        parent = match FolderController::show(user.clone(), *parent_id) {
            Ok(parent) => Some(parent),
            Err(e) => {
        log!(e.level(), "Request from user \"{}\" returned \"{}\"", user.id(), e);
        return Err(Status::from(e));
    },
        };
    }

    let context = EditContext {
        user,
        parent,
        folder,
    };

    Ok(Template::render("folders/edit", &context))
}

#[derive(FromForm)]
pub struct UpdatePayload {
    name: String,
    parent_id: Option<i32>,
}

#[post("/folders/<folder_id>", data = "<payload>")]
pub fn update(auth: Auth, folder_id: i32, payload: Form<UpdatePayload>) -> impl Responder<'static> {
    let user = auth.clone().user();

    match FolderController::update(
        user.clone(),
        folder_id,
        payload.name.to_owned(),
        user.id(),
        payload.parent_id,
    ) {
        Ok(folder) => Ok(Redirect::to(format!("/folders/{}", folder.id()))),
        Err(e) => Err(Status::from(e)),
    }
}

#[post("/folders/<folder_id>/delete")]
pub fn delete(auth: Auth, folder_id: i32) -> impl Responder<'static> {
    let user = auth.user();

    match FolderController::delete(user, folder_id) {
        Ok(folder) => match folder.parent_id() {
            Some(parent_id) => Ok(Redirect::to(format!("/folders/{}", parent_id))),
            None => Ok(Redirect::to("/")),
        },
        Err(e) => Err(Status::from(e)),
    }
}
