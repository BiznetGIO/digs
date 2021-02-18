use log::trace;
use serde::Deserialize;
use std::path::Path;
use toml::de;

use crate::utils;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub servers: Vec<Server>,
}

// Deserialize config intro struct.
fn deserialize_config(content: &str) -> Result<Config, de::Error> {
    let servers: Config = toml::from_str(&content)?;
    Ok(servers)
}

pub fn get_config(path: &Path) -> Config {
    let file_content = utils::read_file(path);
    let config = deserialize_config(&file_content);
    trace!("Config -> {:?}", config);

    match config {
        Ok(conf) => conf,
        Err(_) => {
            eprintln!("Invalid config file.");
            std::process::exit(1);
        }
    }
}
