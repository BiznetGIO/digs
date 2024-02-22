use std::{
    ffi::OsStr,
    fs, net,
    path::{Path, PathBuf},
};

use miette::{NamedSource, Result, SourceOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: net::IpAddr,
    pub name: String,
}

pub fn read<P>(filename: P) -> Result<Config, crate::Error>
where
    P: AsRef<Path>,
    P: AsRef<OsStr>,
{
    let file_content = fs::read_to_string(&filename).map_err(|_| crate::Error::ConfigNotFound {
        path: PathBuf::from(&filename),
    })?;
    deserialize(&file_content, &filename)
}

/// Deserialize config intro struct.
fn deserialize<P>(content: &str, filename: P) -> Result<Config, crate::Error>
where
    P: AsRef<Path>,
    P: AsRef<OsStr>,
{
    match toml::from_str(content) {
        Ok(config) => Ok(config),
        Err(e) => {
            let range = &e.span().unwrap_or(std::ops::Range { start: 0, end: 0 });
            let filename = Path::new(&filename);
            Err(crate::Error::InvalidConfig {
                src: NamedSource::new(filename.to_string_lossy(), content.to_owned()),
                bad_bit: SourceOffset::from_location(content, range.start, range.end),
                message: e.to_string(),
            })?
        }
    }
}
