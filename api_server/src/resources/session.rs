use resources::user::User;

#[derive(Serialize)]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub user: User,
}
