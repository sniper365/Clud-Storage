use db::builders::{Builder, FileBuilder};
use db::models::File;
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
        file_name: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, Error> {
        FileBuilder::new()
            .with_name(name)
            .with_extension(extension)
            .with_file_name(file_name)
            .with_public(public)
            .with_folder_id(folder_id)
            .build()
            .save()
    }

    pub fn update(
        id: i32,
        name: String,
        file_name: String,
        extension: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, Error> {
        let mut file = File::all()
            .filter(files::id.eq(id))
            .first::<File>(&DbFacade::connection())?;

        file.set_name(name);
        file.set_file_name(file_name);
        file.set_extension(extension);
        file.set_folder_id(folder_id);
        file.set_public(public);

        file.update()
    }

    pub fn delete(id: i32) -> Result<File, Error> {
        let file = File::all()
            .filter(files::id.eq(id))
            .first::<File>(&DbFacade::connection())?;

        file.delete()
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
    fn test_create() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let expected = factory!(File, folder.id());

        let actual = FileService::create(
            expected.name().to_string(),
            expected.extension().to_string(),
            expected.file_name().to_string(),
            expected.folder_id(),
            false,
        )?;

        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.extension(), actual.extension());
        assert_eq!(expected.folder_id(), actual.folder_id());

        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let file = factory!(File, folder.id()).save()?;

        let expected = factory!(File, folder.id());
        let actual = FileService::update(
            file.id(),
            expected.name().to_string(),
            expected.file_name().to_string(),
            expected.extension().to_string(),
            expected.folder_id(),
            false,
        )?;

        assert_eq!(file.id(), actual.id());
        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.extension(), actual.extension());
        assert_eq!(expected.folder_id(), actual.folder_id());

        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().expect("Missing .env file");
        let conn = DbFacade::connection();

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let expected = factory!(File, folder.id()).save()?;
        let path = format!(
            "{}/test/{file_name}",
            Env::storage_dir(),
            file_name = &expected.file_name()
        );

        std::fs::File::create(path)?;

        let actual = FileService::delete(expected.id())?;

        let lookup = File::all()
            .filter(files::id.eq(actual.id()))
            .first::<File>(&conn);

        assert_eq!(expected, actual);
        assert_eq!(lookup, Err(diesel::result::Error::NotFound));

        Ok(())
    }
}
