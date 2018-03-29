mod user;
mod new_user;

pub use self::user::User;

use resources::AsResource;
use resources::user::User as UserResource;

impl AsResource for User {
    type Resource = UserResource;

    fn as_resource(&self) -> UserResource {
        UserResource {
            user_id: self.id,
            name: self.name.to_string(),
            email: self.email.to_string(),
            root: self.root,
        }
    }
}
