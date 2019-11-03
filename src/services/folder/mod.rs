pub mod implementation;

use db::models::Folder;
use diesel::result::Error;

pub trait FolderService {
    fn create(name: String, user_id: i32, parent_id: Option<i32>) -> Result<Folder, Error>;

    fn update(
        id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error>;

    fn delete(id: i32) -> Result<Folder, Error>;
}
