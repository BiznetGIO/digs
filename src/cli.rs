use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use hickory_client::rr;

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

#[derive(Clone, ValueEnum)]
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

impl Opts {
    pub fn rtype(&self) -> rr::RecordType {
        match self.rtype {
            RecordType::A => rr::RecordType::A,
            RecordType::AAAA => rr::RecordType::AAAA,
            RecordType::CNAME => rr::RecordType::CNAME,
            RecordType::MX => rr::RecordType::MX,
            RecordType::NS => rr::RecordType::NS,
            RecordType::SOA => rr::RecordType::SOA,
            RecordType::TXT => rr::RecordType::TXT,
        }
    }
}

fn is_domain(domain: &str) -> Result<String, String> {
    if domain.contains('.') {
        Ok(domain.to_string())
    } else {
        Err(format!("{} isn't a valid domain", domain))
    }
}
