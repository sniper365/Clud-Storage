use super::ControllerError as Error;
use db::models::{File, User};
use db::DbPool;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use policies::Restricted;
use schema::*;
use services::FileService;

pub struct FileController;

impl FileController {
    pub fn index(user: User, folder_id: i32) -> Result<Vec<File>, Error> {
        if !user.can_index::<File>() {
            return Err(Error::Forbidden);
        }

        let conn = &DbPool::connection();

        match File::all()
            .filter(files::folder_id.eq(folder_id))
            .load(conn)
        {
            Ok(folders) => Ok(folders),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    pub fn show(user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(_) => return Err(Error::InternalServerError),
        };

        match user.can_view(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
    }

    pub fn create(user: User) -> Result<(), Error> {
        match user.can_create::<File>() {
            true => Ok(()),
            false => Err(Error::Forbidden),
        }
    }

    pub fn store(
        user: User,
        name: String,
        extension: String,
        user_id: i32,
        folder_id: i32,
        public: bool,
        bytes: &[u8],
    ) -> Result<File, Error> {
        if !user.can_create::<File>() {
            return Err(Error::Forbidden);
        }

        match FileService::create(name, extension, user_id, folder_id, public, bytes) {
            Ok(file) => Ok(file),
            Err(_) => Err(Error::InternalServerError),
        }
    }

    pub fn edit(user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(_) => return Err(Error::InternalServerError),
        };

        match user.can_modify(found.clone()) {
            true => Ok(found),
            false => Err(Error::Forbidden),
        }
    }

    pub fn update(
        user: User,
        file_id: i32,
        name: String,
        extension: String,
        public: bool,
        folder_id: i32,
    ) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(_) => return Err(Error::InternalServerError),
        };

        if !user.can_modify(found.clone()) {
            return Err(Error::Forbidden);
        }

        match FileService::update(file_id, name, extension, folder_id, public) {
            Ok(file) => Ok(file),
            Err(_) => return Err(Error::InternalServerError),
        }
    }

    pub fn delete(user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(_) => return Err(Error::InternalServerError),
        };

        if !user.can_delete(found.clone()) {
            return Err(Error::Forbidden);
        }

        match FileService::delete(file_id) {
            Ok(file) => Ok(file),
            Err(_) => return Err(Error::InternalServerError),
        }
    }

    pub fn contents(user: User, file_id: i32) -> Result<Vec<u8>, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(_) => return Err(Error::InternalServerError),
        };

        if !user.can_view(found.clone()) {
            return Err(Error::Forbidden);
        }

        match FileService::contents(file_id) {
            Ok(contents) => Ok(contents),
            Err(_) => return Err(Error::InternalServerError),
        }
    }
}
