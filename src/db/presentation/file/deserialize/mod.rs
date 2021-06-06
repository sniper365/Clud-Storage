mod field;
mod field_visitor;
mod file_visitor;

use self::file_visitor::FileVisitor;
use db::models::File;
use db::presentation::FromJson;
use serde::de::{Deserialize, Deserializer};

const FIELDS: &'static [&'static str] = &["file_id", "name", "folder_id", "public", "extension"];

impl<'de> Deserialize<'de> for File {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("File", FIELDS, FileVisitor)
    }
}

impl<'a> FromJson<'a> for File {}
