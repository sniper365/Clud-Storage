use db::models::{File, Folder};
use db::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

impl File {
    pub fn folder(&self) -> Result<Folder, Error> {
        Folder::all()
            .filter(folders::id.eq(self.folder_id()))
            .first::<Folder>(&DbFacade::connection())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::builders::*;
    use db::query::Query;
    use std::error::Error;

    #[test]
    fn test_folder() -> Result<(), Box<Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let file = factory!(File, folder.id()).save()?;

        let expected = folder;
        let actual = file.folder()?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
