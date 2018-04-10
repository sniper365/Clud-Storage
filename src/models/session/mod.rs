mod session;

pub use self::session::Session;

use resources::AsResource;
use resources::session::Session as SessionResource;

impl AsResource for Session {
    type Resource = SessionResource;

    fn as_resource(&self) -> SessionResource {
        SessionResource {
            user_id: self.user_id,
            token: self.token.clone(),
            user: self.user.as_resource(),
            roles: self.roles.clone().into_iter().map(| role | role.as_resource()).collect()
        }
    }
}
