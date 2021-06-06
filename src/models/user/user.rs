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
use diesel::ExecuteDsl;
use diesel::result::Error;

use models::folder::Folder;
use models::role_user::RoleUser;
use models::role::Role;

use models::user::new_user::NewUser;

#[derive(Queryable, Associations, Identifiable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub root: Option<i32>,
}

impl User {
    pub fn new(name: String, email: String, password: String, root: Option<i32>) -> NewUser {
        use bcrypt::{ DEFAULT_COST, hash };

        NewUser {
            name: name,
            email: email,
            password: hash(&password, DEFAULT_COST).unwrap(),
            root: root,
        }
    }

    pub fn all(conn: &DbConn) -> Result<Vec<User>, Error> {
        use schema::users::dsl::{ users };

        users.load::<User>(conn.deref())
    }

    pub fn find(id: i32, conn: &DbConn) -> Result<User, Error> {
        use schema::users::dsl::{ users };

        users.find(id).first::<User>(conn.deref())
    }

    pub fn save(&self, conn: &DbConn) -> Result<User, Error> {
        use schema::users::dsl::*;

        diesel::update(users.find(&self.id))
            .set((
                name.eq(&self.name),
                email.eq(&self.email),
                password.eq(&self.password),
                root.eq(&self.root)
            ))
            .get_result(conn.deref())
    }

    pub fn delete(&self, conn: &DbConn) -> Result<usize, Error> {
        use schema::users::dsl::users;

        let folders = self.folders(conn)?;

        for folder in folders.into_iter() {
            folder.delete(conn)?;
        }

        let roles = self.role_users(conn)?;

        for role in roles.into_iter() {
            role.delete(conn)?;
        }

        diesel::delete(users.find(&self.id)).execute(conn.deref())
    }

    pub fn root(&self, conn: &DbConn) -> Option<Result<Folder, Error>> {
        use schema::folders::dsl::{ folders, id };

        match self.root {
            Some(root) => Some(folders.filter(id.eq(root)).first::<Folder>(conn.deref())),
            None => None
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

    pub fn roles(&self, conn: &DbConn) -> Result<Vec<Role>, Error> {
        let user_roles = self.role_users(conn)?;

        let result: Result<Vec<_>, _> = user_roles.into_iter().map(| user_role | {
            user_role.role(conn)
        }).collect();

        result
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

    pub fn set_password(&mut self, password: String) {
        use bcrypt::{ DEFAULT_COST, hash };

        self.password = hash(&password, DEFAULT_COST).unwrap();
    }
}
