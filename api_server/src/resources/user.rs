#[derive(Serialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub root: Option<i32>,
}
