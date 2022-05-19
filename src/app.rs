use clap::{crate_version, Arg, Command};

pub fn build() -> Command<'static> {
    let app = Command::new("digs \u{25cf} dig many at once")
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("domain")
                .required(true)
                .help("Domain name to query"),
        )
        .arg(
            Arg::new("rtype")
                .possible_values(&["A", "AAAA", "CNAME", "MX", "NS", "SOA", "TXT"])
                .default_value("A")
                .help("Record Type"),
        )
        .arg(
            Arg::new("config")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Specify an alternate configuration file"),
        );

    app
}
