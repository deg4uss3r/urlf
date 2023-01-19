use confy::ConfyError;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ConfigurationError(ConfyError),
    ErrorParsingCliValue(std::num::ParseIntError),
    WriteError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<ConfyError> for Error {
    fn from(value: ConfyError) -> Self {
        Self::ConfigurationError(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ErrorParsingCliValue(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::WriteError(value)
    }
}

impl error::Error for Error {}
