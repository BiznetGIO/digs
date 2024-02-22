use std::process;

use clap::Parser;
use miette::Result;

use digs::{cli::Opts, output::Printer};

fn run() -> Result<()> {
    let opts = Opts::parse();
    let rtype = opts.rtype();

    let config_file = opts.config;
    let printer = Printer::new(opts.domain, rtype, config_file);
    printer.print()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
