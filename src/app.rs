use clap::{crate_version, App, AppSettings, Arg};

pub fn build() -> App<'static> {
    let app = App::new("digs \u{25cf} dig many at once")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::new("domain")
                .required(true)
                .about("Domain name to query"),
        )
        .arg(
            Arg::new("rtype")
                .possible_values(&["A", "AAAA", "CNAME", "MX", "NS", "SOA", "TXT"])
                .default_value("A")
                .about("Record Type"),
        )
        .arg(
            Arg::new("config")
                .short('f')
                .long("file")
                .takes_value(true)
                .about("Specify an alternate configuration file"),
        );

    app
}
