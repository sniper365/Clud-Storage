#[derive(Deserialize)]
pub struct Store {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Update {
    pub user_id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct Password {
    pub user_id: i64,
    pub current_password: String,
    pub password: String,
    pub password_confirmation: String,
}
