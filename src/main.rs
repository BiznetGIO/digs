#![allow(clippy::wildcard_imports)]

use std::{path::PathBuf, process};
use trust_dns_client::rr::RecordType;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use log::trace;

use digs::{cli, cli::Opts, config, dns, utils};

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

    let config_path: PathBuf = match opts.config {
        Some(path) => utils::is_exist(&path)?,
        None => utils::is_exist("digs.toml")?,
    };
    let config = config::read(&config_path)?;

    for server in config.servers {
        let response = dns::query(&domain, record_type, &server.ip);
        trace!("Response -> {:?}", response);

        println!("{}", server.name);
        match response {
            Err(e) => {
                println!("  {}", e.to_string().red());
            }
            Ok(res) => {
                let print_output = |rr_type: String, name: String, rdata: String| {
                    println!(
                        "  {} {} {}",
                        rr_type.green().bold(),
                        name.blue(),
                        rdata.bold()
                    );
                };

                if !res.answers().is_empty() {
                    for res in res.answers() {
                        let rr_type = res.rr_type().to_string();
                        print_output(
                            rr_type,
                            res.name().to_string(),
                            res.data().context("no `rdata` found")?.to_string(),
                        );
                    }
                } else if res.answers().is_empty() && !res.name_servers().is_empty() {
                    // if answers is empty, print default record (SOA)
                    for res in res.name_servers() {
                        let rr_type = res.rr_type().to_string();
                        print_output(
                            rr_type,
                            res.name().to_string(),
                            res.data().context("no `rdata` found")?.to_string(),
                        );
                    }
                } else {
                    // if default doesn't exist
                    println!("{}", "  No zone found".to_string().red());
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
