#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

pub mod config;
pub mod packet_utils;

use anyhow::Result;
use config::Config;
use packet_utils::{HandshakeRequest, NextState};
use std::env;
use std::io::Cursor;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let config = load_conf();
    debug!("Configuration: {:?}", config);

    start(config).await;
}

async fn start(config: Config) {
    info!("Starting listen on {}", config.get_listen_addr());
    let mut listener = TcpListener::bind(config.get_listen_addr()).await.unwrap();
    let config = Arc::new(config);
    loop {
        let client = accept_client(&mut listener).await;
        if let Err(e) = client {
            error!("Failed to accept a client: {}", e);
            continue;
        }
        let (stream, addr) = client.unwrap();
        debug!("Client connected from {:?}", addr);
        let config = Arc::clone(&config);
        tokio::spawn(async move {
            let result = handle_client(&config, stream, &addr).await;
            if let Err(e) = result {
                error!("{}: An error occurred: {}", addr, e);
            }
        });
    }
}

async fn accept_client(listener: &mut TcpListener) -> Result<(TcpStream, SocketAddr)> {
    let client = listener.accept().await?;
    client.0.set_nodelay(true)?;
    Ok(client)
}

async fn handle_client(config: &Config, mut stream: TcpStream, addr: &SocketAddr) -> Result<()> {
    let handshake = HandshakeRequest::read(&mut stream).await?;
    let redirect_target = config.get_addr_by_host(&handshake.get_host());
    info!(
        "{}: {}: {}:{} -> {}",
        addr,
        handshake.get_next_state(),
        handshake.get_host(),
        handshake.get_port(),
        redirect_target
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    );
    if redirect_target.is_none() {
        if *handshake.get_next_state() == NextState::Login { // Unknown host, disconnect
            write_string(&mut stream, &mut config.get_unknown_host_message()).await?; // Disconnect Message
        } else if *handshake.get_next_state() == NextState::Status { // Unknown host, send unknown host MOTD
            
        }
        return Ok(());
    }
    let redirect_target = redirect_target.unwrap();
    let mut server = TcpStream::connect(redirect_target).await?;
    server.set_nodelay(true)?;
    packet_utils::write_var_int(&mut server, handshake.get_size()).await?;
    server.write_all(handshake.get_raw_body()).await?;

    let (mut client_reader, mut client_writer) = tokio::io::split(stream);
    let (mut server_reader, mut server_writer) = tokio::io::split(server);
    let addr_clone = *addr;
    tokio::spawn(async move {
        let result = tokio::io::copy(&mut client_reader, &mut server_writer).await;
        if let Some(err) = result.err() {
            debug!(
                "{}: An error occurred in client-to-server bridge. Maybe disconnected: {}",
                addr_clone, err
            );
        }
    });
    let result = tokio::io::copy(&mut server_reader, &mut client_writer).await;
    if let Some(err) = result.err() {
        debug!(
            "{}: An error occurred in server-to-client bridge. Maybe disconnected: {}",
            addr, err
        );
    }
    Ok(())
}

async fn write_string(&mut TcpStream, string: &mut &str) -> Result<()> {
    let mut temp: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    packet_utils::write_var_int(&mut temp, 0).await?;
    packet_utils::write_var_int(&mut temp, string.len() as i32).await?;
    temp.write_all(&string.as_bytes()).await?;
    let temp = temp.into_inner();
    packet_utils::write_var_int(stream, temp.len() as i32).await?;
    stream.write_all(&temp).await?;
    Ok(())
}

fn load_conf() -> Config {
    let config_path = Path::new("./config.toml");
    info!("Configuration file: {:?}", config_path);
    Config::load_or_init(config_path)
}
