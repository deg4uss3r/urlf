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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigurationError(confy_error) => write!(
                f,
                "An error occured in the configuration framework: {}",
                confy_error
            ),
            Error::ErrorParsingCliValue(parse_error) => write!(
                f,
                "An error occurred while trying to parse a number: {}",
                parse_error
            ),
            Error::WriteError(io_error) => write!(f, "An error occurred when writing to a file or the terminal: {}", io_error),
            Error::UnableToCreateClient(client_error) => write!(f, "Unable to create the GitLab client: {}", client_error),
            Error::BuilderError(_) => write!(f, "An error occurred when trying to build a request to the GitLab endpoint."),
            Error::EndPointError => write!(f, "An occur occurred from the endpoint, check your provided API key and try again."),
            Error::InvalidArg => write!(f, "An error occurred with the provided arguments, try `--help` for possible arguments."),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigurationError(confy_error) => write!(
                f,
                "An error occured in the configuration framework: {}",
                confy_error
            ),
            Error::ErrorParsingCliValue(parse_error) => write!(
                f,
                "An error occurred while trying to parse a number: {}",
                parse_error
            ),
            Error::WriteError(io_error) => write!(f, "An error occurred when writing to a file or the terminal: {}", io_error),
            Error::UnableToCreateClient(client_error) => write!(f, "Unable to create the GitLab client: {}", client_error),
            Error::BuilderError(_) => write!(f, "An error occurred when trying to build a request to the GitLab endpoint."),
            Error::EndPointError => write!(f, "An occur occurred from the endpoint, check your provided API key and try again."),
            Error::InvalidArg => write!(f, "An error occurred with the provided arguments, try `--help` for possible arguments."),
        }
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
