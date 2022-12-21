use std::path::PathBuf;

use crate::error::Error;

/// # Errors
///
/// Will return `Err` if `domain` is not valid
pub fn is_domain(domain: &str) -> Result<String, Error> {
    if domain.contains('.') {
        Ok(domain.to_string())
    } else {
        Err(Error::InvalidDomain(domain.to_string()))
    }
}

pub fn current_dir() -> Result<PathBuf, Error> {
    Ok(std::env::current_dir()?)
}
