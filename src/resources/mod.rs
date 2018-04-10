pub mod user;
pub mod folder;
pub mod session;
pub mod file;
pub mod role;

use serde_json;
use serde::Serialize;

pub trait AsResource {
    type Resource: Serialize;

    fn as_resource(&self) -> Self::Resource;

    fn as_response(&self) -> String {
        serde_json::to_string(&self.as_resource()).unwrap()
    }
}
