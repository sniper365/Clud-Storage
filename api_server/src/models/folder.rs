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

use models::user::User;
use models::file::File;

#[derive(Queryable, Associations, Identifiable, Serialize)]
#[table_name = "folders"]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "folders"]
pub struct NewFolder {
    pub name: String,
    pub parent_id: Option<i32>,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct Show {
    folder_id: i32,
    name: String,
    parent_id: Option<i32>,
    user_id: i32,
}

impl Folder {
    pub fn new(name: String, parent_id: Option<i32>, user_id: i32) -> NewFolder {
        NewFolder {
            name: name,
            parent_id: parent_id,
            user_id: user_id,
        }
    }

    // Finders
    pub fn find(id: i32, conn: &DbConn) -> Result<Folder, Error> {
        use schema::folders::dsl::{ folders };

        folders.find(id).first::<Folder>(conn.deref())
    }

    pub fn user(&self, conn: &DbConn) -> Result<User, Error> {
        use schema::users::dsl::{ users, id };

        users.filter(id.eq(&self.user_id)).first::<User>(conn.deref())
    }

    pub fn files(&self, conn: &DbConn) -> Result<Vec<File>, Error> {
        use schema::files::dsl::{ files, folder_id };

        files.filter(folder_id.eq(&self.id)).load::<File>(conn.deref())
    }

    pub fn parent(&self, conn: &DbConn) -> Result<Option<Folder>, Error> {
        use schema::folders::dsl::{ folders, id };

        match self.parent_id {
            Some(parent_id) => match folders.filter(id.eq(parent_id)).first::<Folder>(conn.deref()) {
                Ok(folder) => Ok(Some(folder)),
                Err(e) => Err(e),
            }
            None => Ok(None)
        }
    }

    pub fn children(&self, conn: &DbConn) -> Result<Vec<Folder>, Error> {
        use schema::folders::dsl::{ folders, parent_id };

        folders.filter(parent_id.eq(&self.id)).load::<Folder>(conn.deref())
    }

    pub fn save(&self, conn: &DbConn) -> Result<Folder, Error> {
        use schema::folders::dsl::*;

        diesel::update(folders.find(&self.id))
            .set((
                name.eq(&self.name),
                parent_id.eq(self.parent_id),
                user_id.eq(self.user_id)
            ))
            .get_result(conn.deref())
    }

    pub fn delete(&self, conn: &DbConn) -> Result<usize, Error> {
        use schema::folders::dsl::folders;

        let files = self.files(conn)?;

        for file in files.into_iter() {
            file.delete(conn)?;
        }

        diesel::delete(folders.find(&self.id)).execute(conn.deref())
    }

    pub fn into_show(&self) -> Show {
        Show {
            folder_id: self.id,
            name: self.name.to_string(),
            parent_id: self.parent_id,
            user_id: self.user_id,
        }
    }
}

impl NewFolder {
    pub fn save(&self, conn: &DbConn) -> Result<Folder, Error> {
        use std::str::FromStr;
        use schema::folders;

        let new_folder = NewFolder {
            name: String::from_str(&self.name).unwrap(),
            parent_id: self.parent_id,
            user_id: self.user_id
        };

        diesel::insert(&new_folder).into(folders::table).get_result(conn.deref())
    }
}
