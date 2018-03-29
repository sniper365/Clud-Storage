mod file;
mod new_file;

pub use self::file::File;

use resources::AsResource;
use resources::file::File as FileResource;

impl AsResource for File {
    type Resource = FileResource;

    fn as_resource(&self) -> FileResource {
        FileResource {
            file_id: self.id,
            folder_id: self.folder_id,
            name: self.name.to_string(),
            extension: self.extension.to_string(),
            created_at: self.created_at,
        }
    }
}
