use schema::*;

use pg_pool::DbConn;

use diesel;
use diesel::LoadDsl;
use diesel::result::Error;

use std::ops::Deref;

use models::user::User;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub root: Option<i32>,
}

impl NewUser {
    pub fn save(&self, conn: &DbConn) -> Result<User, Error> {
        use schema::users;

        let new_user = NewUser {
            name: self.name.to_string(),
            email: self.email.to_string(),
            password: self.password.to_string(),
            root: self.root,
        };

        diesel::insert(&new_user).into(users::table).get_result(conn.deref())
    }
}
