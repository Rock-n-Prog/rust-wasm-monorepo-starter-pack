pub mod init_tracing;

use clap::Parser;
use futures::{future, prelude::*};
use init_tracing::{init_tracing};
use acme_api_lib::{World};
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
};
use tarpc::{
    context,
    server::{self},
    tokio_serde::formats::Json,
};

#[derive(Parser)]
struct Flags {
    #[clap(long)]
    port: u16,
}

// TODO: Move HelloServer to its own file
#[derive(Clone)]
struct HelloServer(SocketAddr);

#[tarpc::server]
impl World for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {name}! You are connected from {}", self.0)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("ACME tarpc API")?;

    let server_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), flags.port);

    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    tracing::info!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = HelloServer(channel.transport().peer_addr().unwrap());
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}