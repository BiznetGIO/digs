use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub servers: Vec<Server>,
}

pub fn read(path: &Path) -> Result<Config, Error> {
    let file_content = fs::read_to_string(path).map_err(|_| Error::ConfigNotFound {
        path: path.to_path_buf(),
    })?;
    deserialize(&file_content)
}

/// Deserialize config intro struct.
/// # Errors
///
/// Will return `Err` if deserialization error.
/// Possibly the error contains a position (line number) of the occurred error
/// But this is not accurate. All the other apps that depend on toml.rs
/// share the same faith.
fn deserialize(content: &str) -> Result<Config, Error> {
    match toml::from_str(content) {
        Ok(config) => Ok(config),
        Err(e) => Err(Error::InvalidConfig {
            message: e.to_string(),
        }),
    }
}
