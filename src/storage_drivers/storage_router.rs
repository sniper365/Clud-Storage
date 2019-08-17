use super::aws::Aws;
use super::disk::Disk;
use super::storage_driver_option::StorageDriverOption;
use super::StorageDriver;
use env::Env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct StorageRouter;

impl StorageDriver for StorageRouter {
    type Error = StorageRouterError;

    fn store<R>(path: &Path, contents: &mut R) -> Result<(), StorageRouterError>
    where
        R: Read,
    {
        match Env::storage_driver() {
            StorageDriverOption::Aws => {
                Aws::store(path, contents).map_err(|e| StorageRouterError::from(e))
            }
            StorageDriverOption::Disk => {
                Disk::store(path, contents).map_err(|e| StorageRouterError::from(e))
            }
        }
    }

    fn read(path: &Path) -> Result<File, StorageRouterError> {
        match Env::storage_driver() {
            StorageDriverOption::Aws => Aws::read(path).map_err(|e| StorageRouterError::from(e)),
            StorageDriverOption::Disk => Disk::read(path).map_err(|e| StorageRouterError::from(e)),
        }
    }

    fn delete(path: &Path) -> Result<(), StorageRouterError> {
        match Env::storage_driver() {
            StorageDriverOption::Aws => Aws::delete(path).map_err(|e| StorageRouterError::from(e)),
            StorageDriverOption::Disk => {
                Disk::delete(path).map_err(|e| StorageRouterError::from(e))
            }
        }
    }
}

type AwsError = <Aws as StorageDriver>::Error;
type DiskError = <Disk as StorageDriver>::Error;

pub enum StorageRouterError {
    Aws(AwsError),
    Disk(DiskError),
}

impl Error for StorageRouterError {
    fn description(&self) -> &str {
        match self {
            StorageRouterError::Aws(aws) => aws.description(),
            StorageRouterError::Disk(disk) => disk.description(),
        }
    }
}

impl fmt::Display for StorageRouterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageRouterError::Aws(aws) => aws.fmt(f)?,
            StorageRouterError::Disk(disk) => disk.fmt(f)?,
        };

        Ok(())
    }
}

impl fmt::Debug for StorageRouterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageRouterError::Aws(aws) => aws.fmt(f)?,
            StorageRouterError::Disk(disk) => disk.fmt(f)?,
        };

        Ok(())
    }
}

impl From<DiskError> for StorageRouterError {
    fn from(from: DiskError) -> Self {
        StorageRouterError::Disk(from)
    }
}

impl From<AwsError> for StorageRouterError {
    fn from(from: AwsError) -> Self {
        StorageRouterError::Aws(from)
    }
}
