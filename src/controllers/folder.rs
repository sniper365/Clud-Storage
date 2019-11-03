use super::ControllerError as Error;
use db::models::{Folder, User};
use db::DbFacade;
use diesel::result;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use policies::Restricted;
use schema::*;
use services::FolderService;

pub struct FolderController;

impl FolderController {
    pub fn index(user: User, parent_id: Option<i32>) -> Result<Vec<Folder>, Error> {
        if !user.can_index::<Folder>() {
            return Err(Error::Forbidden);
        }

        let conn = &DbFacade::connection();

        let stmt = Folder::all().filter(folders::user_id.eq(user.id()));

        let stmt = match parent_id {
            Some(parent_id) => stmt.filter(folders::parent_id.eq(parent_id)),
            None => stmt.filter(folders::parent_id.is_null()),
        };

        match stmt.load(conn) {
            Ok(folders) => Ok(folders),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    pub fn show(user: User, folder_id: i32) -> Result<Folder, Error> {
        let conn = &DbFacade::connection();

        let found: Folder = match Folder::all().filter(folders::id.eq(&folder_id)).first(conn) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match user.can_view(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
    }

    pub fn create(user: User) -> Result<(), Error> {
        match user.can_create::<Folder>() {
            true => Ok(()),
            false => Err(Error::Forbidden),
        }
    }

    pub fn store(
        user: User,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error> {
        if !user.can_create::<Folder>() {
            return Err(Error::Forbidden);
        }

        match <resolve!(FolderService)>::create(name, user_id, parent_id) {
            Ok(folder) => Ok(folder),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    pub fn edit(user: User, folder_id: i32) -> Result<Folder, Error> {
        let conn = &DbFacade::connection();

        let found: Folder = match Folder::all().filter(folders::id.eq(&folder_id)).first(conn) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match user.can_modify(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
    }

    pub fn update(
        user: User,
        folder_id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error> {
        let conn = &DbFacade::connection();

        let found: Folder = match Folder::all().filter(folders::id.eq(&folder_id)).first(conn) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_modify(found.clone()) {
            return Err(Error::Forbidden);
        }

        match <resolve!(FolderService)>::update(folder_id, name, user_id, parent_id) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }

    pub fn delete(user: User, folder_id: i32) -> Result<Folder, Error> {
        let conn = &DbFacade::connection();

        let found: Folder = match Folder::all().filter(folders::id.eq(&folder_id)).first(conn) {
            Ok(folder) => folder,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_delete(found.clone()) {
            return Err(Error::Forbidden);
        }

        match <resolve!(FolderService)>::delete(folder_id) {
            Ok(folder) => Ok(folder),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }
}
