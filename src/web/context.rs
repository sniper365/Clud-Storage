use super::error::Error;
use serde::ser;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Context<T: ser::Serialize> {
    errors: Vec<Error>,
    data: T,
}

impl<'a, T: ser::Serialize> Context<T> {
    pub fn new(errors: Vec<Error>, data: T) -> Self {
        Self { errors, data }
    }
}
