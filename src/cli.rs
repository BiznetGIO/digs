use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "digs",
    version,
    about = "dig many at once",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/BiznetGIO/digs"
)]
pub struct Opts {
    /// Domain name to query
    #[arg(value_parser = is_domain)]
    pub domain: String,

    /// Record Type
    #[arg(value_enum, default_value_t = RecordType::A)]
    pub rtype: RecordType,

    /// Specify an alternate configuration file
    #[arg(short, long, default_value_os_t = PathBuf::from("digs.toml"))]
    pub config: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    SOA,
    TXT,
}

fn is_domain(domain: &str) -> Result<String, String> {
    if domain.contains('.') {
        Ok(domain.to_string())
    } else {
        Err(format!("{} isn't a valid domain", domain))
    }
}
