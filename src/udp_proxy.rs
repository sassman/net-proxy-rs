use anyhow::Result;
use log::{debug, error, info};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

///
/// a size of 512 byte is considered to be a safe choice
///
/// [read more](https://stackoverflow.com/a/35697810)
/// ### TL;DR
/// > The maximum safe UDP payload is 508 bytes.
/// > This is a packet size of 576 (the "minimum maximum reassembly buffer size"),
/// > minus the maximum 60-byte IP header and the 8-byte UDP header.
///
const UDP_PAYLOAD_SIZE: usize = 512;

pub async fn start_udp_proxy(local_addr: &str, remote_addr: &str) -> Result<()> {
    let mut listener = UdpSocket::bind(&local_addr).await?;
    info!("listening on {:?}://{:?}", crate::Proto::Udp, local_addr);

    let remote_addr = remote_addr.to_owned();
    tokio::spawn(async move {
        match proxy_to_remote(&mut listener, &remote_addr).await {
            Ok(_) => info!("client disconnected"),
            Err(e) => error!("{}", e),
        }
    })
    .await
    .map_err(|e| anyhow::Error::from(e))
}

async fn proxy_to_remote(origin: &mut UdpSocket, remote_addr: &str) -> Result<()> {
    let mut remote = UdpSocket::bind("0.0.0.0:0").await?;
    remote.connect(remote_addr).await?;
    debug!(
        "initializing communication to remote socket from {:?}",
        remote.local_addr().unwrap()
    );

    loop {
        let (payload, peer) = receive_udp_package(origin).await?;
        let forward_response = forward_package(&mut remote, &payload[..]).await?;
        origin.send_to(&forward_response[..], &peer).await?;
    }
}

///
/// forward a udp package to a given sender,
/// wait for the answer and return it
///
async fn forward_package(sender_socket: &mut UdpSocket, payload: &[u8]) -> Result<Vec<u8>> {
    sender_socket.send(payload).await?;
    debug!(
        "sent: {} bytes to {:?}",
        payload.len(),
        sender_socket.local_addr()
    );
    let (result, _) = receive_udp_package(sender_socket).await?;

    Ok(result)
}

///
/// receives a full udp package (datagram)
/// the size of the udp package may vary (MTU),
async fn receive_udp_package(from_socket: &mut UdpSocket) -> Result<(Vec<u8>, SocketAddr)> {
    let mut buf = [0; UDP_PAYLOAD_SIZE];

    let (payload_size, peer) = from_socket.recv_from(&mut buf).await?;
    let filled_buf = &mut buf[..payload_size];
    debug!("received: {} bytes from {:?}", payload_size, &peer);

    return Ok((Vec::from(filled_buf), peer));
}
