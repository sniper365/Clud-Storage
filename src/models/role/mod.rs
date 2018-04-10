mod role;
mod new_role;

pub use self::role::Role;

use resources::AsResource;
use resources::role::Role as RoleResource;

impl AsResource for Role {
    type Resource = RoleResource;

    fn as_resource(&self) -> RoleResource {
        RoleResource {
            role_id: self.id,
            name: self.name.clone(),
        }
    }
}
