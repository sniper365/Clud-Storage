mod folder;
mod new_folder;

pub use self::folder::Folder;

use resources::AsResource;
use resources::folder::Folder as FolderResource;

impl AsResource for Folder {
    type Resource = FolderResource;

    fn as_resource(&self) -> FolderResource {
        FolderResource {
            folder_id: self.id,
            name: self.name.to_string(),
            parent_id: self.parent_id,
            user_id: self.user_id,
        }
    }
}
