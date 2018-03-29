use schema::*;

use pg_pool::DbConn;
use std::ops::Deref;

use diesel;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;
use diesel::FindDsl;
use diesel::LoadDsl;
use diesel::ExecuteDsl;
use diesel::result::Error;

use models::role::Role;
use models::user::User;

#[derive(Queryable, Associations, Identifiable, Serialize)]
#[table_name = "role_user"]
pub struct RoleUser {
    pub id: i32,
    pub role_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "role_user"]
pub struct NewRoleUser {
    pub role_id: i32,
    pub user_id: i32,
}

impl RoleUser {
    pub fn new(role_id: i32, user_id: i32) -> NewRoleUser {
        NewRoleUser {
            role_id: role_id,
            user_id: user_id,
        }
    }

    pub fn user(&self, conn: &DbConn) -> Result<User, Error> {
        use schema::users::dsl::{ users, id };

        users.filter(id.eq(&self.user_id)).first::<User>(conn.deref())
    }

    pub fn role(&self, conn: &DbConn) -> Result<Role, Error> {
        use schema::roles::dsl::{ roles, id };

        roles.filter(id.eq(&self.role_id)).first::<Role>(conn.deref())
    }

    pub fn save(&self, conn: &DbConn) -> Result<RoleUser, Error> {
        use schema::role_user::dsl::*;

        diesel::update(role_user.find(&self.id))
            .set((
                role_id.eq(&self.role_id),
                user_id.eq(&self.user_id)
            ))
            .get_result(conn.deref())
    }

    pub fn delete(&self, conn: &DbConn) -> Result<usize, Error> {
        use schema::role_user::dsl::role_user;

        diesel::delete(role_user.find(&self.id)).execute(conn.deref())
    }
}

impl NewRoleUser {
    pub fn save(&self, conn: &DbConn) -> Result<RoleUser, Error> {
        use schema::role_user;

        let new_role_user = NewRoleUser {
            role_id: self.role_id,
            user_id: self.user_id,
        };

        diesel::insert(&new_role_user).into(role_user::table).get_result(conn.deref())
    }
}
