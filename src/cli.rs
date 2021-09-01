use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, ArgMatches};

pub fn get_cli_args() -> ArgMatches<'static> {
    App::new("net-proxy")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .required(false)
                .help("enables more verbose logging"),
        )
        .arg(
            Arg::with_name("tcp")
                .long("tcp")
                .takes_value(false)
                .required(false)
                .conflicts_with("udp")
                .help("proxy protocol is tcp"),
        )
        .arg(
            Arg::with_name("udp")
                .long("udp")
                .takes_value(false)
                .required(false)
                .conflicts_with("tcp")
                .help("proxy protocol is udp"),
        )
        .arg(
            Arg::with_name("local-port")
                .short("p")
                .long("port")
                .takes_value(true)
                .required(true)
                .default_value("2525")
                .help("local port where the proxy will listening"),
        )
        .arg(
            Arg::with_name("net-server")
                .short("s")
                .long("server")
                .takes_value(true)
                .required(true)
                .help("remote net server address (name or ip)"),
        )
        .arg(
            Arg::with_name("net-server-port")
                .short("P")
                .long("server-port")
                .takes_value(true)
                .required(true)
                .default_value("25")
                .help("remote net server port"),
        )
        .get_matches()
}
