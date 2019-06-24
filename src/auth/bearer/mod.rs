mod error;
mod user;

pub use self::error::Error;

use super::authenticate::Authenticate;
use env::Env;
use frank_jwt::{decode, encode, Algorithm};
use serde::ser::Serialize;
use serde_json::json;
use serde_json::Value;

pub trait Bearer: Sized + Serialize {
    fn header(&self) -> Value;

    fn encode(&self) -> Result<String, Error> {
        let secret = Env::app_key();

        let header = json!(self.header());
        let payload = json!(self);

        let token_string: String = encode(header, &secret, &payload, Algorithm::HS384)?;

        Ok(token_string)
    }

    fn decode(token: &String) -> Result<Value, Error> {
        let secret = Env::app_key();

        match decode(token, &secret, Algorithm::HS384) {
            Ok((_, payload)) => Ok(payload),
            Err(e) => return Err(Error::from(e)),
        }
    }

    fn verify(token: Value) -> Result<Self, Error>;
}

pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Token { token }
    }
}

impl<T: Bearer> Authenticate<T> for Token {
    type Error = Error;

    fn verify(&self) -> Result<T, Self::Error> {
        let decoded = T::decode(&self.token)?;

        T::verify(decoded)
    }
}
