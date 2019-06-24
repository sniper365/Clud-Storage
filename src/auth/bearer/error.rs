use frank_jwt::error::Error as JwtError;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    #[allow(dead_code)]
    KeyMissing,
    SignatureExpired,
    SignatureInvalid,
    JWTInvalid,
    IssuerInvalid,
    ExpirationInvalid,
    AudienceInvalid,
    FormatInvalid(String),
    IoError(String),
    OpenSslError(String),
    ProtocolError(String),
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error {
            JwtError::SignatureExpired => Error::SignatureExpired,
            JwtError::SignatureInvalid => Error::SignatureInvalid,
            JwtError::JWTInvalid => Error::JWTInvalid,
            JwtError::IssuerInvalid => Error::IssuerInvalid,
            JwtError::ExpirationInvalid => Error::ExpirationInvalid,
            JwtError::AudienceInvalid => Error::AudienceInvalid,
            JwtError::FormatInvalid(s) => Error::FormatInvalid(s),
            JwtError::IoError(s) => Error::IoError(s),
            JwtError::OpenSslError(s) => Error::OpenSslError(s),
            JwtError::ProtocolError(s) => Error::ProtocolError(s),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::KeyMissing => write!(f, "App Key Missing"),
            Error::SignatureExpired => write!(f, "Signature expired."),
            Error::SignatureInvalid => write!(f, "Signature invalid."),
            Error::JWTInvalid => write!(f, "JWT invalid."),
            Error::IssuerInvalid => write!(f, "Issuer invalid."),
            Error::ExpirationInvalid => write!(f, "Expiration invalid."),
            Error::AudienceInvalid => write!(f, "Audience invalid."),
            Error::FormatInvalid(msg) => write!(f, "Format invalid: {}.", msg),
            Error::IoError(msg) => write!(f, "IO error: {}.", msg),
            Error::OpenSslError(msg) => write!(f, "Open SSL error: {}.", msg),
            Error::ProtocolError(msg) => write!(f, "Protocol error: {}.", msg),
        }
    }
}

impl error::Error for Error {}
