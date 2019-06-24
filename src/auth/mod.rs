pub mod authenticate;
pub mod basic;
pub mod bearer;
pub mod error;

use self::authenticate::Authenticate;
use self::basic::{Basic, Credentials};
use self::bearer::{Bearer, Token};
use self::error::Error;

pub enum Auth {
    Basic(Credentials),
    Bearer(Token),
}

/// Ideally we'd use negation to allow multiple auth methods,
/// such as a model only implementing Token Auth,
/// however this is currently not possible without negative bounds
/// so until then, waiting on Negative Trait Bounds
impl<T: Basic + Bearer> Authenticate<T> for Auth {
    type Error = Error;

    fn verify(&self) -> Result<T, Error> {
        let auth = match self {
            Auth::Basic(c) => c.verify()?,
            Auth::Bearer(t) => t.verify()?,
        };

        Ok(auth)
    }
}
