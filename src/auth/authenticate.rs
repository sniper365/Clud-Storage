use std::error::Error as StdError;

pub trait Authenticate<T> {
    type Error: StdError;

    fn verify(&self) -> Result<T, Self::Error>;
}
