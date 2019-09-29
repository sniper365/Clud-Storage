mod aws;
mod disk;
pub mod storage_driver_option;
pub mod storage_router;

pub use self::storage_router::StorageRouter;

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub trait StorageDriver {
    type Error: Error;

    fn store(path: &Path, contents: File) -> Result<(), Self::Error>;

    fn read(path: &Path) -> Result<File, Self::Error>;

    fn delete(path: &Path) -> Result<(), Self::Error>;
}
