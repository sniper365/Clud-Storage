pub mod implementation;

use db::models::User;
use diesel::result::Error;

pub trait UserService {
    fn create(
        name: String,
        email: String,
        role: String,
        password: String,
    ) -> Result<User, Error>;

    fn update(id: i32, name: String, email: String, role: String) -> Result<User, Error>;

    fn delete(id: i32) -> Result<User, Error>;

    fn update_password(id: i32, password: String) -> Result<User, Error>;
}
