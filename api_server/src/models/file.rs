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

#[derive(Queryable, Associations, Identifiable, Serialize)]
#[table_name = "files"]
pub struct File {
    pub id: i32,
    pub name: String,
    pub file_name: String,
    pub folder_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub extension: String,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub name: String,
    pub file_name: String,
    pub extension: String,
    pub folder_id: i32,
}

#[derive(Serialize)]
pub struct Show {
    file_id: i32,
    folder_id: i32,
    name: String,
    extension: String,
}

impl File {
    pub fn new(name: String, file_name: String, folder_id: i32, extension: String) -> NewFile {
        NewFile {
            name: name,
            file_name: file_name,
            extension: extension,
            folder_id: folder_id,
        }
    }

    // Finders
    pub fn find(id: i32, conn: &DbConn) -> Result<File, Error> {
        use schema::files::dsl::{ files };

        files.find(id).first::<File>(conn.deref())
    }

    pub fn folders(&self, conn: &DbConn) -> Result<Folder, Error> {
        use schema::folders::dsl::{ folders, id };

        folders.filter(id.eq(&self.folder_id)).first::<Folder>(conn.deref())
    }

    pub fn save(&self, conn: &DbConn) -> Result<File, Error> {
        use schema::files::dsl::*;

        diesel::update(files.find(&self.id))
            .set((
                name.eq(&self.name),
                file_name.eq(&self.file_name),
                extension.eq(&self.extension),
                folder_id.eq(self.folder_id)
            ))
            .get_result(conn.deref())
    }

    pub fn delete(&self, conn: &DbConn) -> Result<usize, Error> {
        use schema::files::dsl::files;

        diesel::delete(files.find(&self.id)).execute(conn.deref())
    }

    pub fn into_show(&self) -> Show {
        Show {
            file_id: self.id,
            folder_id: self.folder_id,
            name: self.name.to_string(),
            extension: self.extension.to_string(),
        }
    }
}

impl NewFile {
    pub fn save(&self, conn: &DbConn) -> Result<File, Error> {
        use std::str::FromStr;
        use schema::files;

        let new_file = NewFile {
            name: String::from_str(&self.name).unwrap(),
            file_name: String::from_str(&self.file_name).unwrap(),
            extension: String::from_str(&self.extension).unwrap(),
            folder_id: self.folder_id
        };

        diesel::insert(&new_file).into(files::table).get_result(conn.deref())
    }
}
