mod error;
mod user;

pub use self::error::Error;
use super::authenticate::Authenticate;

pub trait Basic: Sized {
    fn verify(identifier: String, key: String) -> Result<Self, error::Error>;
}

pub struct Credentials {
    identity: String,
    key: String,
}

impl Credentials {
    pub fn new(identity: String, key: String) -> Self {
        Self { identity, key }
    }
}

impl<T: Basic> Authenticate<T> for Credentials {
    type Error = Error;

    fn verify(&self) -> Result<T, Self::Error> {
        let identity = &self.identity.clone();
        let key = &self.key.clone();

        T::verify(identity.to_owned(), key.to_owned())
    }
}
