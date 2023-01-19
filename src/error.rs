use confy::ConfyError;
use gitlab::{api::projects::merge_requests::MergeRequestChangesBuilderError, GitlabError};

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ConfigurationError(ConfyError),
    ErrorParsingCliValue(std::num::ParseIntError),
    WriteError(std::io::Error),
    UnableToCreateClient(GitlabError),
    BuilderError(MergeRequestChangesBuilderError),
    EndPointError,
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

impl From<GitlabError> for Error {
    fn from(value: GitlabError) -> Self {
        Self::UnableToCreateClient(value)
    }
}

impl From<MergeRequestChangesBuilderError> for Error {
    fn from(value: MergeRequestChangesBuilderError) -> Self {
        Self::BuilderError(value)
    }
}

impl error::Error for Error {}
