use super::ControllerError as Error;
use db::models::{File, User};
use db::DbPool;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use policies::Restricted;
use schema::*;
use services::{FileService, StorageService};
use std::fs;
use std::io::Read;

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

    pub fn store<R>(
        user: User,
        name: String,
        extension: String,
        user_id: i32,
        folder_id: i32,
        public: bool,
        input: &mut R,
    ) -> Result<File, Error>
    where
        R: Read,
    {
        if !user.can_create::<File>() {
            return Err(Error::Forbidden);
        }

        let file_name = StorageService::store(user_id.to_string(), input).unwrap();

        match FileService::create(name, extension, file_name, folder_id, public) {
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

        match FileService::update(
            file_id,
            found.file_name().to_string(),
            name,
            extension,
            folder_id,
            public,
        ) {
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

        if let Err(_) = StorageService::delete(user.id().to_string(), found.file_name().to_string())
        {
            return Err(Error::InternalServerError);
        }

        match FileService::delete(file_id) {
            Ok(file) => Ok(file),
            Err(_) => return Err(Error::InternalServerError),
        }
    }

    pub fn contents(user: User, file_id: i32) -> Result<fs::File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(_) => return Err(Error::InternalServerError),
        };

        if !user.can_view(found.clone()) {
            return Err(Error::Forbidden);
        }

        match StorageService::read(user.id().to_string(), found.file_name().to_string()) {
            Ok(contents) => Ok(contents),
            Err(_) => return Err(Error::InternalServerError),
        }
    }
}
