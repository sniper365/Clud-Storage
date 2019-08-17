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
    match Env::aws_bucket_region().parse() {
        Ok(region) => Ok(region),
        Err(e) => {
            log!("error", "Invalid AWS Region: {}", e);
            Err(e)
        }
    }
}

fn bucket() -> Result<Bucket, S3Error> {
    match Bucket::new(&Env::aws_bucket_name(), region()?, credentials()) {
        Ok(bucket) => Ok(bucket),
        Err(e) => {
            log!("error", "Invalid AWS Bucket: {}", e);
            Err(e)
        }
    }
}

pub struct Aws;

impl StorageDriver for Aws {
    type Error = AwsError;

    fn store<R>(path: &Path, contents: &mut R) -> Result<(), Self::Error>
    where
        R: Read,
    {
        let bucket = match bucket() {
            Ok(bucket) => bucket,
            Err(e) => {
                log!("error", "Failed to create bucket: {}", e);
                return Err(AwsError::from(e));
            }
        };

        let mut buffer = Vec::new();

        log!("warn", "Using non-streamed body");
        if let Err(e) = contents.read_to_end(&mut buffer) {
            log!("error", "Failed to read contents: {}", e);
            return Err(AwsError::from(e));
        }

        if let Err(e) = bucket.put_object(path.to_str().unwrap(), buffer.as_slice(), "text/plain") {
            log!("error", "Failed to download object from S3: {}", e);
            return Err(AwsError::from(e));
        }

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

        // Creating scope block here to force the file to close, and reopen,
        //  meaning the file cannot be opened while still being owned by the process
        {
            let mut file = match File::create(Path::new(&tmp)) {
                Ok(file) => file,
                Err(e) => {
                    log!("error", "Failed to create tmp file: {}", e);
                    return Err(AwsError::from(e));
                }
            };

            let bucket = match bucket() {
                Ok(bucket) => bucket,
                Err(e) => {
                    log!("error", "Failed to create bucket: {}", e);
                    return Err(AwsError::from(e));
                }
            };

            if let Err(e) = bucket.get_object_stream(path.to_str().unwrap(), &mut file) {
                log!("error", "Failed to get object from AWS: {}", e);
                return Err(AwsError::from(e));
            }
        }

        let file = match File::open(Path::new(&tmp)) {
            Ok(file) => file,
            Err(e) => {
                log!("error", "Failed to open tmp file: {}", e);
                return Err(AwsError::from(e));
            }
        };

        Ok(file)
    }

    fn delete(path: &Path) -> Result<(), Self::Error> {
        let bucket = match bucket() {
            Ok(bucket) => bucket,
            Err(e) => {
                log!("error", "Failed to create bucket: {}", e);
                return Err(AwsError::from(e));
            }
        };

        match bucket.delete_object(path.to_str().unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => {
                log!("error", "Failed to delete object from AWS: {}", e);
                return Err(AwsError::from(e));
            }
        }
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
