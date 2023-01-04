use std::process;
use trust_dns_client::rr::RecordType;

use clap::Parser;
use miette::Result;

use digs::{cli, cli::Opts, error::Error, output::Printer};

fn run() -> Result<()> {
    let opts = Opts::parse();

    let domain = is_domain(&opts.domain)?;
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
    let printer = Printer::new(domain, record_type, config_file);
    printer.print()?;
    Ok(())
}

fn is_domain(domain: &str) -> Result<String, Error> {
    if domain.contains('.') {
        Ok(domain.to_string())
    } else {
        Err(Error::InvalidDomain(domain.to_string()))
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
