use std::path::{Path, PathBuf};

use crate::error::DigsError;

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

/// # Errors
///
/// Will return `Err` if `domain` is not valid
pub fn is_domain(domain: &str) -> Result<String, DigsError> {
    if domain.contains('.') {
        Ok(domain.to_string())
    } else {
        Err(DigsError::InvalidDomain(domain.to_string()))
    }
}

/// # Errors
///
/// Will return `Err` if `path` does not exist
pub fn is_exist(path: &str) -> Result<PathBuf, DigsError> {
    let path = Path::new(path);

    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err(DigsError::NoFile(path.to_path_buf()))
    }
}
