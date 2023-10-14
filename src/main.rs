use hickory_client::rr::RecordType;
use std::process;

use clap::Parser;
use miette::Result;

use digs::{cli, cli::Opts, output::Printer};

fn run() -> Result<()> {
    let opts = Opts::parse();

    let record_type = match opts.rtype {
        cli::RecordType::A => RecordType::A,
        cli::RecordType::AAAA => RecordType::AAAA,
        cli::RecordType::CNAME => RecordType::CNAME,
        cli::RecordType::MX => RecordType::MX,
        cli::RecordType::NS => RecordType::NS,
        cli::RecordType::SOA => RecordType::SOA,
        cli::RecordType::TXT => RecordType::TXT,
    };

    let config_file = opts.config;
    let printer = Printer::new(opts.domain, record_type, config_file);
    printer.print()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
