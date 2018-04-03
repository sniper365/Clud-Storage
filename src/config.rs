use std::env;

#[allow(dead_code)]
pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be defined")
}

#[allow(dead_code)]
pub fn app_key() -> String {
    env::var("APP_KEY").expect("APP_KEY must be defined")
}

#[allow(dead_code)]
pub fn build_dir() -> String {
    match env::var("BUILD_DIR") {
        Ok(var) => var,
        Err(_) => String::from("frontend/build"),
    }
}

#[allow(dead_code)]
pub fn app_index() -> String {
    match env::var("APP_INDEX") {
        Ok(var) => var,
        Err(_) => String::from("index.html"),
    }
}

#[allow(dead_code)]
pub fn storage_dir() -> String {
    match env::var("STORAGE_DIR") {
        Ok(var) => var,
        Err(_) => String::from("storage"),
    }
}
