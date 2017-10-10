use ::chrono::*;
use schema::*;

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
