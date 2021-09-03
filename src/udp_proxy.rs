use anyhow::Result;
use log::{debug, error, info};
use tokio::io;
use tokio::net::UdpSocket;

///
/// the size of the udp package may vary (MTU)
/// a size of 2048 byte seems to be a safe choice
///
/// [read more](https://stackoverflow.com/a/35697810)
/// ### TL;DR
/// > The maximum safe UDP payload is 508 bytes.
/// > This is a packet size of 576 (the "minimum maximum reassembly buffer size"),
/// > minus the maximum 60-byte IP header and the 8-byte UDP header.
///
/// Apparently things work differently
///
const UDP_PAYLOAD_SIZE: usize = 2048;

pub async fn start_udp_proxy(local_addr: &str, remote_addr: &str) -> Result<()> {
    let mut listener = UdpSocket::bind(&local_addr).await?;
    info!("listening on {:?}://{}", crate::Proto::Udp, local_addr);

    let remote_addr = remote_addr.to_owned();
    tokio::spawn(async move {
        match proxy_to_remote(&mut listener, &remote_addr).await {
            Ok(_) => info!("client disconnected"),
            Err(e) => error!("{}", e),
        }
    })
    .await
    .map_err(anyhow::Error::from)
}

async fn proxy_to_remote(origin: &mut UdpSocket, remote_addr: &str) -> Result<()> {
    let mut remote = UdpSocket::bind("0.0.0.0:0").await?;
    remote.connect(remote_addr).await?;
    info!("initializing communication to remote {}", remote_addr);

    loop {
        match handle_incoming_package(origin, &mut remote).await {
            Ok(_) => debug!("one package transmitted"),
            Err(e) => error!("{}", e),
        }
    }
}

async fn handle_incoming_package(origin: &mut UdpSocket, remote: &mut UdpSocket) -> Result<()> {
    let mut buf = [0; UDP_PAYLOAD_SIZE];

    // receive a package from the client / origin
    let (payload_size, peer) = origin.recv_from(&mut buf).await?;
    let payload = &buf[..payload_size];
    debug!("received: {} bytes from origin", payload.len());

    // forward the origin package to the remote side
    remote.send(payload).await?;
    debug!("Client->Remote: {} bytes", payload.len());

    // on the remote side we might not even have data available
    // so we don't block here
    match remote.try_recv_from(&mut buf) {
        Ok((payload_size, _)) => {
            let payload = &buf[..payload_size];
            debug!("received: {} bytes from remote", payload.len());

            // send the answer from remote to origin
            origin.send_to(payload, &peer).await?;
            debug!("Remote->Client: {} bytes", payload.len());

            Ok(())
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}
