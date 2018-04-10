use schema::*;

use pg_pool::DbConn;
use std::ops::Deref;

use diesel;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FindDsl;
use diesel::FirstDsl;
use diesel::LoadDsl;
use diesel::result::Error;

use models::role_user::RoleUser;
use models::role::new_role::NewRole;

#[derive(Queryable, Associations, Identifiable, Serialize, Clone)]
#[table_name = "roles"]
#[belongs_to(RoleUser)]
pub struct Role {
    pub id: i32,
    pub name: Option<String>,
}

impl Role {
    pub fn new(name: String) -> NewRole {
        NewRole {
            name: name,
        }
    }

    // Finders
    pub fn find(id: i32, conn: &DbConn) -> Result<Role, Error> {
        use schema::roles::dsl::{ roles };

        roles.find(id).first::<Role>(conn.deref())
    }

    pub fn role_users(&self, conn: &DbConn) -> Result<Vec<RoleUser>, Error> {
        use schema::role_user::dsl::{ role_user, role_id };

        role_user.filter(role_id.eq(&self.id)).load::<RoleUser>(conn.deref())
    }

    pub fn save(&self, conn: &DbConn) -> Result<Role, Error> {
        use schema::roles::dsl::*;

        diesel::update(roles.find(&self.id))
            .set(name.eq(&self.name))
            .get_result(conn.deref())
    }
}
