use std::io::{self, Write};

use hickory_client::rr;
use log::trace;
use owo_colors::OwoColorize;

use crate::{config::Config, dns};

pub struct Printer {
    domain: String,
    record_type: rr::RecordType,
    config: Config,
}

impl Printer {
    pub fn new(domain: String, record_type: rr::RecordType, config: Config) -> Self {
        Self {
            domain,
            record_type,
            config,
        }
    }
    pub fn print(&self) -> Result<(), crate::Error> {
        for server in &self.config.servers {
            let response = dns::query(&self.domain, self.record_type, server.address);
            trace!("Response -> {:?}", response);

            stdout(&server.name.to_string());
            match response {
                Err(e) => {
                    stdout(&format!("  {}", e.to_string().red()));
                }
                Ok(res) => {
                    if !res.answers().is_empty() {
                        for record in res.answers() {
                            Self::print_record(record);
                        }
                    } else if res.answers().is_empty() && !res.name_servers().is_empty() {
                        // If answers is empty, print default record (SOA)
                        for record in res.name_servers() {
                            Self::print_record(record);
                        }
                    } else {
                        // If default doesn't exist
                        stdout(&format!("{}", "  No zone found".to_string().red()));
                    }
                }
            }
        }
        Ok(())
    }
    fn print_record(record: &rr::Record) {
        let record_type = record.record_type().to_string();
        let name = record.name().to_string();
        let rdata = match record.data() {
            Some(rdata) => rdata.to_string(),
            None => "".to_string(),
        };
        let rdata = rdata.bold();
        stdout(&format!(
            "  {} {} {}",
            record_type.green().bold(),
            name.blue(),
            rdata
        ));
    }
}

pub fn stdout(input: &str) {
    writeln!(io::stdout(), "{input}").ok();
}

pub fn stderr(input: &str) {
    writeln!(io::stderr(), "{input}").ok();
}
