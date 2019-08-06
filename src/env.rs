use logging::{LogLevel, LoggerOption};
use std::convert::TryFrom;
use std::env;

#[allow(dead_code)]
const DATABASE_URL: &str = "DATABASE_URL";

#[allow(dead_code)]
const TEST_DATABASE_URL: &str = "TEST_DATABASE_URL";

#[allow(dead_code)]
const APP_KEY: &str = "APP_KEY";

#[allow(dead_code)]
const STORAGE_DIR: &str = "STORAGE_DIR";

#[allow(dead_code)]
const BCRYPT_COST: &str = "BCRYPT_COST";

#[allow(dead_code)]
const STREAM_CHUNK_SIZE: &str = "STREAM_CHUNK_SIZE";

#[allow(dead_code)]
const LOG_LEVEL: &str = "LOG_LEVEL";

#[allow(dead_code)]
const LOGGER: &str = "LOGGER";

pub struct Env;

#[allow(dead_code)]
impl Env {
    #[cfg(not(test))]
    pub fn database_url() -> String {
        match env::var(DATABASE_URL) {
            Ok(database_url) => database_url,
            Err(e) => panic!(e),
        }
    }

    #[cfg(test)]
    pub fn database_url() -> String {
        match env::var(TEST_DATABASE_URL) {
            Ok(database_url) => database_url,
            Err(e) => panic!(e),
        }
    }

    pub fn app_key() -> String {
        match env::var(APP_KEY) {
            Ok(app_key) => app_key,
            Err(e) => panic!(e),
        }
    }

    pub fn storage_dir() -> String {
        match env::var(STORAGE_DIR) {
            Ok(storage_dir) => storage_dir,
            Err(_) => String::from("storage"),
        }
    }

    pub fn bcrypt_cost() -> u32 {
        match env::var(BCRYPT_COST) {
            Ok(bcrypt_cost) => bcrypt_cost.parse::<u32>().unwrap(),
            Err(e) => panic!(e),
        }
    }

    pub fn chunk_size() -> u64 {
        match env::var(STREAM_CHUNK_SIZE) {
            Ok(bcrypt_cost) => bcrypt_cost.parse::<u64>().unwrap(),
            Err(e) => panic!(e),
        }
    }

    pub fn log_level() -> LogLevel {
        match env::var(LOG_LEVEL) {
            Ok(result) => match LogLevel::try_from(result) {
                Ok(log_level) => log_level,
                Err(e) => panic!(e),
            },
            Err(_) => LogLevel::Error,
        }
    }

    pub fn logger() -> LoggerOption {
        match env::var(LOGGER) {
            Ok(logger) => match LoggerOption::try_from(logger) {
                Ok(logger) => logger,
                Err(e) => panic!(e),
            },
            Err(_) => LoggerOption::default(),
        }
    }
}
