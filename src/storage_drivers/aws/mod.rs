use super::StorageDriver;
use chrono::Utc;
use env::Env;
use rand::{self, distributions::Alphanumeric, Rng};
use s3::bucket::Bucket;
use s3::credentials::Credentials;
use s3::error::S3Error;
use s3::region::Region;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn credentials() -> Credentials {
    Credentials::new(
        Some(Env::aws_access_key_id()),
        Some(Env::aws_access_key_secret()),
        None,
        None,
    )
}

fn region() -> Result<Region, S3Error> {
    Env::aws_bucket_region().parse()
}

fn bucket() -> Result<Bucket, S3Error> {
    Bucket::new(&Env::aws_bucket_name(), region()?, credentials())
}

pub struct Aws;

impl StorageDriver for Aws {
    type Error = AwsError;

    fn store<R>(path: &Path, contents: &mut R) -> Result<(), Self::Error>
    where
        R: Read,
    {
        let bucket = bucket()?;

        let mut buffer = Vec::new();
        contents.read_to_end(&mut buffer).unwrap();
        bucket.put_object(path.to_str().unwrap(), buffer.as_slice(), "text/plain")?;

        Ok(())
    }

    fn read(path: &Path) -> Result<File, Self::Error> {
        let timestamp = Utc::now().to_string();
        let random_bytes: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .collect();

        let tmp = format!(
            "/tmp/{timestamp}_{random_bytes}",
            timestamp = timestamp,
            random_bytes = random_bytes
        );

        {
            let mut file = File::create(Path::new(&tmp))?;

            let bucket = bucket()?;

            bucket.get_object_stream(path.to_str().unwrap(), &mut file)?;
        }

        Ok(File::open(Path::new(&tmp))?)
    }

    fn delete(path: &Path) -> Result<(), Self::Error> {
        let bucket = bucket()?;

        bucket.delete_object(path.to_str().unwrap())?;

        Ok(())
    }
}

pub enum AwsError {
    S3(S3Error),
    IO(Error),
}

impl error::Error for AwsError {
    fn description(&self) -> &str {
        match self {
            AwsError::S3(s3) => s3.description(),
            AwsError::IO(io) => io.description(),
        }
    }
}

impl fmt::Display for AwsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            AwsError::S3(s3) => s3.fmt(f),
            AwsError::IO(io) => io.fmt(f),
        }
    }
}

impl fmt::Debug for AwsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            AwsError::S3(s3) => s3.fmt(f),
            AwsError::IO(io) => io.fmt(f),
        }
    }
}

impl From<S3Error> for AwsError {
    fn from(from: S3Error) -> Self {
        AwsError::S3(from)
    }
}

impl From<Error> for AwsError {
    fn from(from: Error) -> Self {
        AwsError::IO(from)
    }
}
