use libraries::jwt::Token;
use models::user::User;
use models::role::Role;

#[derive(Serialize)]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub user: User,
    pub roles: Vec<Role>,
}

impl Session {
    pub fn new(token: Token, user: User, roles: Vec<Role>) -> Self {
        Session {
            user_id: user.id,
            token: token.encode(),
            user: user,
            roles: roles
        }
    }
}
