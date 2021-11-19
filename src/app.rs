use clap::{crate_version, App, AppSettings, Arg};

pub fn build() -> App<'static, 'static> {
    let app = App::new("digs \u{25cf} dig many at once")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::with_name("domain")
                .required(true)
                .help("Domain name to query"),
        )
        .arg(
            Arg::with_name("rtype")
                .possible_values(&["A", "AAAA", "CNAME", "MX", "NS", "SOA", "TXT"])
                .default_value("A")
                .help("Record Type"),
        )
        .arg(
            Arg::with_name("config")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Specify an alternate configuration file"),
        );

    app
}
