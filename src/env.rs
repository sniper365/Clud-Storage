use std::env;

#[allow(dead_code)]
const DATABASE_URL: &str = "DATABASE_URL";

#[allow(dead_code)]
const TEST_DATABASE_URL: &str = "TEST_DATABASE_URL";

#[allow(dead_code)]
const APP_KEY: &str = "APP_KEY";

#[allow(dead_code)]
const STORAGE_DIR: &str = "STORAGE_DIR";

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
}
