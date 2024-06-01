use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer as _,
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    listen_addr: String,
    upstream_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = resolve_config();
    let config = Arc::new(config);
    info!("Config: {:?}", config);

    let listenser = TcpListener::bind(&config.listen_addr).await?;
    loop {
        let (client, addr) = listenser.accept().await?;
        info!("Accepted connection from: {:?}", addr);
        let cloned_config = Arc::clone(&config);

        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_config.upstream_addr).await?;
            info!("Connected to upstream: {:?}", cloned_config.upstream_addr);

            proxy(client, upstream).await?;
            Ok::<_, anyhow::Error>(())
        });
    }

    Ok(())
}

fn resolve_config() -> Config {
    Config {
        listen_addr: "0.0.0.0:8081".to_string(),
        upstream_addr: "0.0.0.0:8080".to_string(),
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> Result<()> {
    let (mut client_read, mut client_write) = client.split();
    let (mut upstream_read, mut upstream_write) = upstream.split();

    let client_to_upstream = tokio::io::copy(&mut client_read, &mut upstream_write);
    let upstream_to_client = tokio::io::copy(&mut upstream_read, &mut client_write);

    if let Err(e) = tokio::try_join!(client_to_upstream, upstream_to_client) {
        warn!("Error: {:?}", e);
    }
    Ok(())
}
