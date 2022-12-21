use std::process;
use trust_dns_client::rr::RecordType;

use clap::Parser;
use miette::Result;

use digs::{cli, cli::Opts, output::Printer, utils};

fn run() -> Result<()> {
    let opts = Opts::parse();

    let domain = utils::is_domain(&opts.domain)?;
    let record_type = match opts.rtype {
        cli::RecordType::A => RecordType::A,
        cli::RecordType::AAAA => RecordType::AAAA,
        cli::RecordType::CNAME => RecordType::CNAME,
        cli::RecordType::MX => RecordType::MX,
        cli::RecordType::NS => RecordType::NS,
        cli::RecordType::SOA => RecordType::SOA,
        cli::RecordType::TXT => RecordType::TXT,
    };

    let printer = Printer::new(domain, record_type, opts.config);
    printer.print()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
