use db::builders::Builder;
use db::models::File;

pub struct FileBuilder {
    name: String,
    file_name: String,
    folder_id: i32,
    extension: String,
}

impl Default for FileBuilder {
    fn default() -> Self {
        Self {
            name: String::default(),
            file_name: String::default(),
            folder_id: 0,
            extension: String::default(),
        }
    }
}

#[allow(dead_code)]
impl FileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn with_folder_id(mut self, folder_id: i32) -> Self {
        self.folder_id = folder_id;

        self
    }

    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = file_name;

        self
    }

    pub fn with_extension(mut self, extension: String) -> Self {
        self.extension = extension;

        self
    }
}

impl Builder for FileBuilder {
    type Out = File;

    fn build(self) -> Self::Out {
        let mut file = File::new();

        file.set_name(self.name);
        file.set_folder_id(self.folder_id);
        file.set_extension(self.extension);
        file.set_file_name(self.file_name);

        file
    }
}
