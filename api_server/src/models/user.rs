use ::chrono::*;
use schema::*;

use pg_pool::DbConn;
use std::ops::Deref;

use diesel;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;
use diesel::FindDsl;
use diesel::LoadDsl;
use diesel::result::Error;

use models::folder::Folder;
use models::role_user::RoleUser;
use models::role::Role;

#[derive(Queryable, Associations, Identifiable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(first_name: String, last_name: String, email: String, password: String) -> NewUser {
        use bcrypt::{ DEFAULT_COST, hash };

        NewUser {
            first_name: first_name,
            last_name: last_name,
            email: email,
            password: hash(&password, DEFAULT_COST).unwrap(),
        }
    }

    pub fn folders(&self, conn: &DbConn) -> Result<Vec<Folder>, Error> {
        use schema::folders::dsl::{ folders, user_id };

        folders.filter(user_id.eq(&self.id)).load::<Folder>(conn.deref())
    }

    pub fn role_users(&self, conn: &DbConn) -> Result<Vec<RoleUser>, Error> {
        use schema::role_user::dsl::{ role_user, user_id };

        role_user.filter(user_id.eq(&self.id)).load::<RoleUser>(conn.deref())
    }

    pub fn save(&self, conn: &DbConn) -> Result<User, Error> {
        use schema::users::dsl::*;

        diesel::update(users.find(&self.id))
            .set((
                first_name.eq(&self.first_name),
                last_name.eq(&self.last_name),
                email.eq(&self.email),
                password.eq(&self.password),
                token.eq(&self.token)
            ))
            .get_result(conn.deref())
    }

    pub fn is_admin(&self, conn: &DbConn) -> bool {
        use schema::role_user::dsl::*;
        use schema::roles::dsl::*;

        let admin = match roles.filter(name.eq("admin"))
        .first::<Role>(conn.deref()) {
            Ok(role) => role,
            Err(_) => return false,
        };

        match role_user.filter(user_id.eq(&self.id))
        .filter(role_id.eq(admin.id))
        .first::<RoleUser>(conn.deref()) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}

impl NewUser {
    pub fn save(&self, conn: &DbConn) -> Result<User, Error> {
        use std::str::FromStr;
        use schema::users;

        let new_user = NewUser {
            first_name: String::from_str(&self.first_name).unwrap(),
            last_name: String::from_str(&self.last_name).unwrap(),
            email: String::from_str(&self.email).unwrap(),
            password: String::from_str(&self.password).unwrap(),
        };

        diesel::insert(&new_user).into(users::table).get_result(conn.deref())
    }
}
