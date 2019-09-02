use super::error::Error;
use super::success::Success;
use serde::ser;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Context<T: ser::Serialize> {
    errors: Vec<Error>,
    successes: Vec<Success>,
    data: T,
}

impl<'a, T: ser::Serialize> Context<T> {
    pub fn new(errors: Vec<Error>, successes: Vec<Success>, data: T) -> Self {
        Self {
            errors,
            successes,
            data,
        }
    }
}
