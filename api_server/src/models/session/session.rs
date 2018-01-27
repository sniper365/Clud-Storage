use libraries::jwt::Token;
use models::user::User;

#[derive(Serialize)]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub user: User,
}

impl Session {
    pub fn new(token: Token, user: User) -> Self {
        Session {
            user_id: user.id,
            token: token.encode(),
            user: user,
        }
    }
}
