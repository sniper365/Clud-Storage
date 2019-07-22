use controllers::{FileController, FolderController};
use db::models::{File, Folder, User};
use rocket::get;
use rocket::http::Status;
use rocket::response::Responder;
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use web::guards::auth::Auth;

#[derive(Serialize)]
struct HomeContext {
    user: User,
    folder: Folder,
    folders: Vec<Folder>,
    files: Vec<File>,
}

#[get("/")]
pub fn home(auth: Auth) -> impl Responder<'static> {
    let user = auth.to_owned().user();

    let folder = match FolderController::index(user.clone(), None) {
        Ok(folders) => match folders.first() {
            Some(root) => root.to_owned(),
            None => return Err(Status::InternalServerError),
        },
        Err(e) => return Err(Status::from(e)),
    };

    let folders = match FolderController::index(user.clone(), Some(folder.id())) {
        Ok(folders) => folders,
        Err(e) => return Err(Status::from(e)),
    };

    let files = match FileController::index(user.clone(), folder.id()) {
        Ok(files) => files,
        Err(e) => return Err(Status::from(e)),
    };

    let context = HomeContext {
        user,
        folder,
        folders,
        files,
    };

    Ok(Template::render("home", &context))
}
