use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Error(String, String);

impl Error {
    pub fn error(&self) -> &String {
        &self.0
    }

    pub fn message(&self) -> &String {
        &self.1
    }

    pub fn to_string(&self) -> String {
        format!("{}::{}", self.error(), self.message()).to_string()
    }
}

impl From<&str> for Error {
    fn from(from: &str) -> Self {
        let mut parts = from.split("::");

        Error(
            parts.nth(0).unwrap_or("").to_string(),
            parts.nth(0).unwrap_or("").to_string(),
        )
    }
}
