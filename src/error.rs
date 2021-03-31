#![allow(clippy::module_name_repetitions)]

use std::path::PathBuf;

use thiserror::Error;
use trust_dns_client::error::ClientError;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum DigsError {
    #[error("Invalid domain {0:?}")]
    InvalidDomain(String),

    #[error("No such file {0:?}")]
    NoFile(PathBuf),

    #[error("Invalid config")]
    InvalidConfig { source: toml::de::Error },

    #[error("Invalid IP address {0:?}")]
    InvalidIpAddress(String),

    #[error("Network is unreachable")]
    NetworkError,

    // All cases from trust-dns
    #[error("Error: {0:?}")]
    ForeignError(#[from] ClientError),
}
