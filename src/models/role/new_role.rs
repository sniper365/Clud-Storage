use schema::*;

use pg_pool::DbConn;
use std::ops::Deref;

use diesel;
use diesel::LoadDsl;
use diesel::result::Error;

use models::role::role::Role;

#[derive(Insertable)]
#[table_name = "roles"]
pub struct NewRole {
    pub name: String,
}

impl NewRole {
    pub fn save(&self, conn: &DbConn) -> Result<Role, Error> {
        use std::str::FromStr;
        use schema::roles;

        let new_role = NewRole {
            name: String::from_str(&self.name).unwrap(),
        };

        diesel::insert(&new_role).into(roles::table).get_result(conn.deref())
    }
}
