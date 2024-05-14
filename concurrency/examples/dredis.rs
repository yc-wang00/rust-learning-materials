use std::io;

use anyhow::Result;
use tokio::{io::AsyncWriteExt as _, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6380";
    // build a listener
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    loop {
        let (socket, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(socket).await {
                warn!("error processing connection; error = {:?}", e);
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn process_redis_conn(mut stream: tokio::net::TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let data = String::from_utf8_lossy(&buf);
                info!("data: {}", data);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
