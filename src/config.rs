use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

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

pub fn read<P>(filename: P) -> Result<Config, Error>
where
    P: AsRef<Path> + AsRef<OsStr>,
{
    let file_content = fs::read_to_string(&filename).map_err(|_| Error::ConfigNotFound {
        path: PathBuf::from(&filename),
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
