use super::context::Context;
use super::error::Error;
use rocket::http::{Cookie, Cookies};
use serde::Serialize;

pub struct State<'a>(Cookies<'a>);

impl<'a> State<'a> {
    pub fn cookies(self) -> Cookies<'a> {
        self.0
    }

    pub fn errors(&mut self) -> Vec<Error> {
        match &self.0.get_private("errors") {
            Some(cookie) => {
                let errors: Vec<Error> =
                    cookie.value().split("||").map(|e| Error::from(e)).collect();

                &self.0.remove_private(Cookie::named("errors"));

                errors
            }
            None => Vec::new(),
        }
    }

    pub fn push_error(&mut self, error: Error) {
        let mut errors = self.errors();

        errors.push(error);

        let cookie_value = errors
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("||");

        self.0.add_private(Cookie::new("errors", cookie_value));
    }

    pub fn into_context<T: Serialize>(mut self, data: T) -> Context<T> {
        Context::new(self.errors(), data)
    }
}

impl<'a> From<Cookies<'a>> for State<'a> {
    fn from(from: Cookies<'a>) -> Self {
        Self(from)
    }
}
