use anyhow::Result;
use log::{error, info};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn start_tcp_proxy(local_addr: &str, remote_addr: &str) -> Result<()> {
    let listener = TcpListener::bind(&local_addr).await?;
    info!("listening on {:?}://{:?}", crate::Proto::Tcp, local_addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let remote_addr = remote_addr.to_owned();
        tokio::spawn(async move {
            match proxy_to_remote(stream, remote_addr.as_str()).await {
                Ok(_) => info!("client disconnected"),
                Err(e) => error!("{}", e),
            }
        });
    }
}

async fn proxy_to_remote(mut origin: TcpStream, remote: &str) -> Result<()> {
    let mut remote = TcpStream::connect(remote).await?;

    // TODO(timeout): tokio `TcpStream` don't have them, find a different way
    // remote.set_read_timeout(Some(Duration::from_secs(10)));
    // origin.set_read_timeout(Some(Duration::from_secs(10)))?;

    // remote
    //     .set_nodelay(true)
    //     .context("failed to set nodelay to remote")?;
    // origin
    //     .set_nodelay(true)
    //     .context("failed to set nodelay to origin")?;

    let (mut ri, mut wi) = origin.split();
    let (mut ro, mut wo) = remote.split();

    let local_to_remote = async {
        // tokio::io::copy(&mut ri, &mut wo).await?;
        proxy_reader_writer("Client->Remote:", &mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let remote_to_local = async {
        proxy_reader_writer("Remote->Client:", &mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(local_to_remote, remote_to_local)?;

    Ok(())
}

async fn proxy_reader_writer<'a, R: ?Sized, W: ?Sized>(
    direction: &str,
    reader: &'a mut R,
    writer: &'a mut W,
) -> std::io::Result<()>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
{
    let bytes_submitted = tokio::io::copy(reader, writer).await?;
    info!("{} {}", direction, bytes_submitted);

    Ok(())
}
