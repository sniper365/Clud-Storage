use frank_jwt::{Header, Payload, Algorithm, encode, decode };

use frank_jwt::error::Error;

pub struct Token {
    user_id: i32,
    name: String,
}

impl Token {
    pub fn new(user_id: i32, name: String) -> Self {
        Self {
            user_id: user_id,
            name: name,
        }
    }

    pub fn from_encoded(jwt: String) -> Result<(Header, Payload), Error> {
        let secret = env!("APP_KEY");

        decode(jwt, secret.to_string(), Algorithm::HS384)
    }

    pub fn encode(&self) -> String {
        let mut payload = Payload::new();
        payload.insert("user_id".to_string(), self.user_id.to_string());
        payload.insert("name".to_string(), self.name.to_string());

        let secret = env!("APP_KEY");

        let header = Header::new(Algorithm::HS384);

        encode(header, secret.to_string(), payload)
    }
}
