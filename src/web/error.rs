use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Error(String);

impl Error {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn message(&self) -> &String {
        &self.0
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.message()).to_string()
    }
}

impl From<&str> for Error {
    fn from(from: &str) -> Self {
        Error(from.to_string())
    }
}
