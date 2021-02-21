use clap::{crate_version, App, AppSettings, Arg};
use colored::*;
use log::trace;
use std::path::PathBuf;

use digs::{config, dns, utils};

fn main() {
    pretty_env_logger::init_custom_env("DIGS_DEBUG");
    let default_config_path: String = utils::current_dir()
        .join("digs.toml")
        .into_os_string()
        .into_string()
        .unwrap();

    let matches = App::new("digs ● dig many at once")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::new("domain")
                .required(true)
                .validator(utils::is_domain)
                .about("Domain name to query"),
        )
        .arg(
            Arg::new("rtype")
                .possible_values(&["A", "AAAA", "CNAME", "MX", "NS", "SOA", "TXT"])
                .default_value("A"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .default_value(&default_config_path)
                .validator(utils::is_exist)
                .about("Specify an alternate configuration file"),
        )
        .get_matches();

    let mut domain = "";
    let mut config_path: PathBuf = default_config_path.into();
    let rtype = matches.value_of_t("rtype").unwrap_or_else(|e| e.exit());

    if let Some(domain_) = matches.value_of("domain") {
        domain = domain_;
    }
    if let Some(path) = matches.value_of("file") {
        config_path = path.into();
    }

    let config = config::get_config(&config_path);
    trace!("Config -> {:?}", config);

    for server in config.servers {
        let response = dns::query(domain, rtype, &server.ip);
        trace!("Response -> {:?}", response);

        println!("{}", server.name);
        match response {
            Err(e) => {
                println!("  {}", e.to_string().red())
            }
            Ok(res) => {
                for res in res.answers() {
                    let rr_type = res.rr_type().to_string().green().bold();
                    println!(
                        "  {0: <15} {1: <15} {2: <10}",
                        rr_type.to_string(),
                        res.name().to_string().blue(),
                        res.rdata().to_string().bold()
                    );
                }
            }
        }
    }
}
