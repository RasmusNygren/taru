use core::fmt;
use std::{error, io};

#[derive(Debug)]
pub enum TaruError {
    InvalidJob,
    ParseError,
    RuntimeError,
}

impl fmt::Display for TaruError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TaruError::InvalidJob => write!(f, "Invalid job"),
            TaruError::ParseError => write!(f, "Parse Error"),
            TaruError::RuntimeError => write!(f, "Runtime Error"),
        }
    }
}

impl From<io::Error> for TaruError {
    fn from(_err: io::Error) -> Self {
        TaruError::ParseError
    }
}

impl error::Error for TaruError {}
