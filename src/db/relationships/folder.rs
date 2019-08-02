use db::models::{File, Folder, User};
use db::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

impl Folder {
    pub fn files(&self) -> Result<Vec<File>, Error> {
        File::all()
            .filter(files::folder_id.eq(self.id()))
            .load::<File>(&DbFacade::connection())
    }

    pub fn user(&self) -> Result<User, Error> {
        User::all()
            .filter(users::id.eq(self.user_id()))
            .first::<User>(&DbFacade::connection())
    }
}

#[cfg(test)]
mod tests {
    use db::builders::*;
    use db::query::Query;
    use std::error::Error;

    #[test]
    fn test_files() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;
        let expected = vec![
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
            factory!(File, folder.id()).save()?,
        ];

        let actual = folder.files()?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_user() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save()?;
        let folder = factory!(Folder, user.id(), None).save()?;

        let expected = user;
        let actual = folder.user()?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
