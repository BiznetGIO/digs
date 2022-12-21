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
    pub domain: String,

    /// Record Type
    #[arg(value_enum, default_value_t = RecordType::A)]
    pub rtype: RecordType,

    /// Specify an alternate configuration file
    #[arg(short = 'f', long = "file", default_value_os_t = PathBuf::from("digs.toml"))]
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
