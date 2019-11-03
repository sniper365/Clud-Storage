pub mod implementation;

use db::models::File;
use diesel::result::Error;

pub trait FileService {
    fn create(
        name: String,
        extension: String,
        file_name: String,
        folder_id: i32,
        public: bool
    ) -> Result<File, Error>;

    fn update(
        id: i32,
        name: String,
        file_name: String,
        extension: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, Error>;

    fn delete(id: i32) -> Result<File, Error>;
}
