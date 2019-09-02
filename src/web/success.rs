use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Success(String);

impl Success {
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

impl From<&str> for Success {
    fn from(from: &str) -> Self {
        Success(from.to_string())
    }
}
