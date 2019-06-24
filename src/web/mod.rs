use rocket::{catchers, routes};

mod guards;
mod handlers;

use self::handlers::*;
use controllers::ControllerError;
use rocket::http::Status;
use rocket_contrib::templates::Template;

pub fn boot() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                auth::login,
                auth::authenticate,
                home::home,
                folder::index,
                folder::show,
                folder::create,
                folder::store,
                folder::edit,
                folder::update,
                folder::delete,
                file::index,
                file::show,
                file::create,
                file::store,
                file::edit,
                file::update,
                file::delete,
                file::download,
                user::index,
                user::show,
                user::create,
                user::store,
                user::edit,
                user::update,
                user::delete,
                admin::index
            ],
        )
        .register(catchers![error::unauthorized])
        .launch();
}

impl From<ControllerError> for Status {
    fn from(error: ControllerError) -> Self {
        match error {
            ControllerError::Unauthorized => Status::Unauthorized,
            ControllerError::Forbidden => Status::Forbidden,
            ControllerError::NotFound => Status::NotFound,
            ControllerError::InternalServerError => Status::InternalServerError,
        }
    }
}
