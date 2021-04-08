#![allow(clippy::wildcard_imports)]

use std::env;
use std::path::PathBuf;
use std::process;

use anyhow::Result;
use colored::*;
use log::trace;

use digs::app;
use digs::{config, dns, utils};

fn run() -> Result<()> {
    let matches = app::build().get_matches_from(env::args_os());

    // get domain
    // domain must be present. unwrap safe here
    let domain = utils::is_domain(matches.value_of("domain").unwrap())?;

    // get rtype
    // must be present. unwrap safe here
    let rtype = matches.value_of_t("rtype").unwrap();

    // get config file
    let config_path: PathBuf = match matches.value_of("config") {
        Some(path) => utils::is_exist(path)?,
        None => utils::is_exist("digs.toml")?,
    };
    let config = config::get(&config_path)?;

    for server in config.servers {
        let response = dns::query(&domain, rtype, &server.ip);
        trace!("Response -> {:?}", response);

        println!("{}", server.name);
        match response {
            Err(e) => {
                println!("  {}", e.to_string().red())
            }
            Ok(res) => {
                let print_output = |rr_type: String, name: String, rdata: String| {
                    println!(
                        "  {0: <15} {1: <15} {2: <10}",
                        rr_type.green().bold(),
                        name.blue(),
                        rdata.bold()
                    );
                };

                for res in res.answers() {
                    let rr_type = res.rr_type().to_string();
                    print_output(rr_type, res.name().to_string(), res.rdata().to_string());
                }
                for res in res.name_servers() {
                    let rr_type = res.rr_type().to_string();
                    print_output(rr_type, res.name().to_string(), res.rdata().to_string());
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
