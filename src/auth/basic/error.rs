use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    CredentialsInvalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CredentialsInvalid => write!(f, "Invalid Credentials"),
        }
    }
}

impl error::Error for Error {}
