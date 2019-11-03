use db::builders::{Builder, FolderBuilder};
use db::models::Folder;
use db::query::Query;
use db::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

pub struct FolderService;

impl FolderService {
    pub fn create(name: String, user_id: i32, parent_id: Option<i32>) -> Result<Folder, Error> {
        let folder = FolderBuilder::new()
            .with_name(name)
            .with_user_id(user_id)
            .with_parent_id(parent_id)
            .build();

        folder.save()
    }

    pub fn update(
        id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error> {
        let mut folder = Folder::all()
            .filter(folders::id.eq(id))
            .first::<Folder>(&DbFacade::connection())?;

        folder.set_name(name);
        folder.set_user_id(user_id);
        folder.set_parent_id(parent_id);

        folder.update()
    }

    pub fn delete(id: i32) -> Result<Folder, Error> {
        let folder = Folder::all()
            .filter(folders::id.eq(id))
            .first::<Folder>(&DbFacade::connection())?;

        folder.delete()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::builders::*;
    use db::DbFacade;

    #[test]
    fn test_create() {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save().unwrap();
        let expected = factory!(Folder, user.id(), None);

        let actual = FolderService::create(
            expected.name().to_string(),
            expected.user_id(),
            *expected.parent_id(),
        )
        .unwrap();

        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.user_id(), actual.user_id());
        assert_eq!(expected.parent_id(), actual.parent_id());
    }

    #[test]
    fn test_update() {
        dotenv::dotenv().expect("Missing .env file");

        let user = factory!(User).save().unwrap();
        let folder = factory!(Folder, user.id(), None).save().unwrap();

        let expected = factory!(Folder, user.id(), None);
        let actual = FolderService::update(
            folder.id(),
            expected.name().to_string(),
            expected.user_id(),
            *expected.parent_id(),
        )
        .unwrap();

        assert_eq!(folder.id(), actual.id());
        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.user_id(), actual.user_id());
        assert_eq!(expected.parent_id(), actual.parent_id());
    }

    #[test]
    fn test_delete() {
        dotenv::dotenv().expect("Missing .env file");
        let conn = DbFacade::connection();

        let user = factory!(User).save().unwrap();
        let expected = factory!(Folder, user.id(), None).save().unwrap();
        let actual = FolderService::new().delete(expected.id()).unwrap();

        let lookup = Folder::all()
            .filter(folders::id.eq(actual.id()))
            .first::<Folder>(&conn);

        assert_eq!(expected, actual);
        assert_eq!(lookup, Err(Error::NotFound));
    }
}
