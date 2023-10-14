use std::path::PathBuf;

use miette::{Diagnostic, NamedSource, SourceOffset};
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("Configuration file is not found `{path}`")]
    #[diagnostic(
        code(digs::no_config),
        url(docsrs),
        help("Try creating a config file. See https://github.com/BiznetGIO/digs#usage")
    )]
    ConfigNotFound { path: PathBuf },

    #[error("Invalid configuration: {message}")]
    #[diagnostic(
        code(digs::invalid_config),
        url(docsrs),
        help("See the configuration example https://github.com/BiznetGIO/digs#usage")
    )]
    InvalidConfig {
        #[source_code]
        src: NamedSource,
        #[label("{message}")]
        bad_bit: SourceOffset,
        message: String,
    },
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<hickory_client::error::ClientError> for Error {
    fn from(err: hickory_client::error::ClientError) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<hickory_client::proto::error::ProtoError> for Error {
    fn from(err: hickory_client::proto::error::ProtoError) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
