pub enum StorageDriverOption {
    Aws,
    Disk,
}

impl From<String> for StorageDriverOption {
    fn from(from: String) -> Self {
        match from.to_lowercase().as_str() {
            "aws" => StorageDriverOption::Aws,
            "disk" => StorageDriverOption::Disk,
            _ => panic!("Not a valid storage driver"),
        }
    }
}
