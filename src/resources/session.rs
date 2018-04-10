use resources::user::User;
use resources::role::Role;

#[derive(Serialize)]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub user: User,
    pub roles: Vec<Role>,
}
