use std::path::PathBuf;

use colored::Colorize;
use log::trace;
use trust_dns_client::rr::{Record, RecordType};

use crate::error::Error;
use crate::{config, dns};

pub struct Printer {
    domain: String,
    record_type: RecordType,
    config: PathBuf,
}

impl Printer {
    pub fn new(domain: String, record_type: RecordType, config: PathBuf) -> Self {
        Self {
            domain,
            record_type,
            config,
        }
    }
    pub fn print(&self) -> Result<(), Error> {
        let config = config::read(&self.config)?;

        for server in config.servers {
            let response = dns::query(&self.domain, self.record_type, &server.ip);
            trace!("Response -> {:?}", response);

            println!("{}", server.name);
            match response {
                Err(e) => {
                    println!("  {}", e.to_string().red());
                }
                Ok(res) => {
                    if !res.answers().is_empty() {
                        for record in res.answers() {
                            Self::print_record(record);
                        }
                    } else if res.answers().is_empty() && !res.name_servers().is_empty() {
                        // if answers is empty, print default record (SOA)
                        for record in res.name_servers() {
                            Self::print_record(record);
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
    fn print_record(record: &Record) {
        let record_type = record.record_type().to_string();
        let record_type = record_type.green().bold();
        let name = record.name().to_string().blue();
        let rdata = match record.data() {
            Some(rdata) => rdata.to_string(),
            None => "".to_string(),
        };
        let rdata = rdata.bold();
        println!("  {} {} {}", record_type, name, rdata);
    }
}
