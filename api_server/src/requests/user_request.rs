#[derive(Deserialize)]
pub struct Store {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
