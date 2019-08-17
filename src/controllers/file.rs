use super::ControllerError as Error;
use db::models::{File, User};
use db::DbPool;
use diesel::result;
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
            log!(
                "info",
                "403 Forbidden. Indexing Files not allowed for user {}",
                user.id()
            );
            return Err(Error::Forbidden);
        }

        let conn = &DbPool::connection();

        match File::all()
            .filter(files::folder_id.eq(folder_id))
            .load(conn)
        {
            Ok(folders) => Ok(folders),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    pub fn show(user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        match user.can_view(found.clone()) {
            true => Ok(found),
            false => {
                log!(
                    "info",
                    "403 Forbidden. Viewing Files not allowed for user {}",
                    user.id()
                );
                Err(Error::Forbidden)
            }
        }
    }

    pub fn create(user: User) -> Result<(), Error> {
        match user.can_create::<File>() {
            true => Ok(()),
            false => {
                log!(
                    "info",
                    "403 Forbidden. Creating Files not allowed for user {}",
                    user.id()
                );
                Err(Error::Forbidden)
            }
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
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }

    pub fn edit(user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
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
        file_id: i32,
        name: String,
        extension: String,
        public: bool,
        folder_id: i32,
    ) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_modify(found.clone()) {
            return Err(Error::Forbidden);
        }

        match FileService::update(
            file_id,
            name,
            found.file_name().to_string(),
            extension,
            folder_id,
            public,
        ) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }

    pub fn delete(user: User, file_id: i32) -> Result<File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_delete(found.clone()) {
            log!(
                "info",
                "403 Forbidden. Deletion not allowed for user {}",
                user.id()
            );
            return Err(Error::Forbidden);
        }

        if let Err(_) = StorageService::delete(user.id().to_string(), found.file_name().to_string())
        {
            return Err(Error::InternalServerError);
        }

        match FileService::delete(file_id) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }

    pub fn contents(user: User, file_id: i32) -> Result<fs::File, Error> {
        let conn = &DbPool::connection();

        let found: File = match File::all().filter(files::id.eq(&file_id)).first(conn) {
            Ok(file) => file,
            Err(result::Error::NotFound) => return Err(Error::NotFound),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        let owner: i32 = match found.folder() {
            Ok(folder) => folder.user_id(),
            Err(e) => {
                log!("error", "500 Internal Server Erro: {}", e);
                return Err(Error::InternalServerError);
            }
        };

        if !user.can_view(found.clone()) {
            return Err(Error::Forbidden);
        }

        match StorageService::read(owner.to_string(), found.file_name().to_string()) {
            Ok(contents) => Ok(contents),
            Err(e) => {
                log!("error", "500 Internal Server Error: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::builders::*;
    use db::query::Query;
    use std::error::Error;

    #[test]
    fn test_index() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let mut expected = vec![
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
        ];

        let mut actual = FileController::index(user, folder.id())?;

        // Sorting the lists, Vec will return != if they are in
        //  different order, but this shouldn't care
        expected.sort_by(|l, r| l.id().cmp(&r.id()));
        actual.sort_by(|l, r| l.id().cmp(&r.id()));

        assert_eq!(expected, actual);

        Ok(())
    }
}
