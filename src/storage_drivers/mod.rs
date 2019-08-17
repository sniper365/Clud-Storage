mod aws;
mod disk;
pub mod storage_driver_option;
pub mod storage_router;

pub use self::storage_router::StorageRouter;

use self::aws::Aws;
use self::disk::Disk;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub trait StorageDriver {
    type Error: Error;

    fn store<R>(path: &Path, contents: &mut R) -> Result<(), Self::Error>
    where
        R: Read;

    fn read(path: &Path) -> Result<File, Self::Error>;

    fn delete(path: &Path) -> Result<(), Self::Error>;
}
