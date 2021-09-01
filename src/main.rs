use crate::tcp_proxy::start_tcp_proxy;
use crate::udp_proxy::start_udp_proxy;
use anyhow::Result;
use clap::ArgMatches;
use log::debug;

mod cli;
mod tcp_proxy;
mod udp_proxy;

#[derive(Debug)]
pub enum Proto {
    Tcp,
    Udp,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = cli::get_cli_args();
    init_logger(&args);

    let local_addr = format!("0.0.0.0:{}", args.value_of("local-port").unwrap());
    let remote_addr = format!(
        "{}:{}",
        args.value_of("net-server").unwrap(),
        args.value_of("net-server-port").unwrap()
    );
    debug!("remote: {}", remote_addr);

    if args.is_present("udp") {
        debug!("protocol: {:?}", Proto::Udp);
        start_udp_proxy(&local_addr, &remote_addr).await
    } else {
        debug!("protocol: {:?}", Proto::Tcp);
        start_tcp_proxy(&local_addr, &remote_addr).await
    }
}

fn init_logger(args: &ArgMatches) {
    if args.is_present("verbose") {
        std::env::set_var("RUST_LOG", "debug");
    }
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}
