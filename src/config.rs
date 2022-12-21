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
    let file_content = fs::read_to_string(path)?;
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
    toml::from_str(content).map_err(|e| Error::InvalidConfig { source: e })
}
