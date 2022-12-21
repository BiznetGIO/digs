use std::path::{Path, PathBuf};

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

/// # Errors
///
/// Will return `Err` if `path` does not exist
pub fn is_exist(path: &str) -> Result<PathBuf, Error> {
    let path = Path::new(path);

    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err(Error::NoFile(path.to_path_buf()))
    }
}

pub fn current_dir() -> Result<PathBuf, Error> {
    Ok(std::env::current_dir()?)
}
