pub mod user;
pub mod folder;
pub mod session;
pub mod file;

pub trait AsResource {
    type Resource;

    fn as_resource(&self) -> Self::Resource;
}
