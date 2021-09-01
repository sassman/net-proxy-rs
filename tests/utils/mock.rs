use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

const PROXY_ADDR: &str = "127.0.0.1:10443";

pub struct Mock {
    pub remote: UdpSocket,
    pub proxy_addr: SocketAddr,
}

///
/// get an udp socket that acts as remote
///
pub fn remote() -> Mock {
    let mut remote = UdpSocket::bind("0.0.0.0:44310").unwrap();
    remote.set_read_timeout(Some(Duration::from_secs(5)));
    Mock {
        remote,
        proxy_addr: PROXY_ADDR.parse().unwrap(),
    }
}

///
/// get an udp socket that acts as the client
///
pub fn client() -> UdpSocket {
    let addr = "0.0.0.0:0".parse::<SocketAddr>().unwrap();
    let mut c = UdpSocket::bind(&addr).unwrap();
    c.connect(PROXY_ADDR.parse::<SocketAddr>().unwrap())
        .unwrap();

    c
}
