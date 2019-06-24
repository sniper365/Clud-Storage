use super::StorageService;
use db::builders::{Builder, FileBuilder};
use db::models::{File, Folder};
use db::query::Query;
use db::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

pub struct FileService;

impl FileService {
    pub fn create(
        name: String,
        extension: String,
        user_id: i32,
        folder_id: i32,
        bytes: &[u8],
    ) -> Result<File, Error> {
        let file_name = StorageService::store(user_id.to_string(), bytes).unwrap();

        FileBuilder::new()
            .with_name(name)
            .with_extension(extension)
            .with_file_name(file_name)
            .with_folder_id(folder_id)
            .build()
            .save()
    }

    pub fn update(id: i32, name: String, extension: String, folder_id: i32) -> Result<File, Error> {
        let mut file = File::all()
            .filter(files::id.eq(id))
            .first::<File>(&DbFacade::connection())?;

        file.set_name(name);
        file.set_extension(extension);
        file.set_folder_id(folder_id);

        file.update()
    }

    pub fn delete(id: i32) -> Result<File, Error> {
        let file = File::all()
            .filter(files::id.eq(id))
            .first::<File>(&DbFacade::connection())?;

        file.delete()
    }

    pub fn contents(id: i32) -> Result<Vec<u8>, Error> {
        let file = File::all()
            .filter(files::id.eq(id))
            .first::<File>(&DbFacade::connection())?;

        let folder = Folder::all()
            .filter(folders::id.eq(file.folder_id()))
            .first::<Folder>(&DbFacade::connection())?;

        let contents =
            StorageService::read(folder.user_id().to_string(), file.file_name().to_string())
                .unwrap();

        Ok(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::builders::*;
    use db::DbFacade;
    use env::Env;
    use std::error::Error;

    #[test]
    fn test_create() -> Result<(), Box<Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let expected = factory!(File, folder.id());

        let actual = FileService::create(
            expected.name().to_string(),
            expected.extension().to_string(),
            folder.user_id(),
            expected.folder_id(),
            Vec::new().as_slice(),
        )?;

        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.extension(), actual.extension());
        assert_eq!(expected.folder_id(), actual.folder_id());

        let path = format!(
            "{}/test/{file_name}",
            Env::storage_dir(),
            file_name = &actual.file_name()
        );

        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), Box<Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let file = factory!(File, folder.id()).save()?;

        let expected = factory!(File, folder.id());
        let actual = FileService::update(
            file.id(),
            expected.name().to_string(),
            expected.extension().to_string(),
            folder.user_id(),
            expected.folder_id(),
            Vec::new().as_slice(),
        )?;

        assert_eq!(file.id(), actual.id());
        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.extension(), actual.extension());
        assert_eq!(expected.folder_id(), actual.folder_id());

        let path = format!(
            "{}/test/{file_name}",
            Env::storage_dir(),
            file_name = &actual.file_name()
        );

        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), Box<Error>> {
        dotenv::dotenv().expect("Missing .env file");
        let conn = DbFacade::connection();

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let expected = factory!(File, folder.id()).save()?;
        let actual = FileService::delete(expected.id())?;

        let lookup = File::all()
            .filter(files::id.eq(actual.id()))
            .first::<File>(&conn);

        assert_eq!(expected, actual);
        assert_eq!(lookup, Err(diesel::result::Error::NotFound));

        Ok(())
    }
}
