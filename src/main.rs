use std::process;

use clap::Parser;
use miette::Result;

use digs::{cli::Opts, config, config::Config, exit_codes::ExitCode, output, output::Printer};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            output::stderr(&format!("Error: {err:?}"));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let opts = Opts::parse();
    let rtype = opts.rtype();
    let domain = opts.domain.clone();
    let config = construct_config(opts)?;

    let printer = Printer::new(domain, rtype, config);
    printer.print()?;

    Ok(ExitCode::Success)
}

fn construct_config(opts: Opts) -> Result<Config, digs::Error> {
    config::read(opts.config)
}
