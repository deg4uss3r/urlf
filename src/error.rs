use confy::ConfyError;
use gitlab::{
    api::projects::{issues::IssueBuilderError, merge_requests::MergeRequestChangesBuilderError},
    GitlabError,
};

use std::error;
use std::fmt;

pub enum Error {
    ConfigurationError(ConfyError),
    ErrorParsingCliValue(std::num::ParseIntError),
    WriteError(std::io::Error),
    UnableToCreateClient(GitlabError),
    BuilderError(Box<dyn BuilderError>),
    EndPointError,
    InvalidArg,
}

pub trait BuilderError {}
impl BuilderError for MergeRequestChangesBuilderError {}
impl BuilderError for IssueBuilderError {}

// TODO proper Debug/Display for individual error types
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for Error {
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
        Self::BuilderError(Box::new(value))
    }
}

impl From<IssueBuilderError> for Error {
    fn from(value: IssueBuilderError) -> Self {
        Self::BuilderError(Box::new(value))
    }
}

impl error::Error for Error {}
