pub mod admin;
pub mod auth;
pub mod file;
pub mod folder;
pub mod home;
pub mod public;
pub mod user;

use rocket::get;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/resources/<file..>")]
pub fn resources(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("resources/").join(file)).ok()
}
