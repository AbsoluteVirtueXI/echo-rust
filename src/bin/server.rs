use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{StreamExt};
use std::process::exit;

use echo_rust::easy_net::*;
use async_trait::async_trait;

/// main
#[tokio::main]
async fn main() -> io::Result<()> {
    let mut server = match TcpServer::bind("127.0.0.1:7777").await {
        Ok(server) => {
            println!("Server bound.");
            server
        },
        Err(e) => {
            println!("Server can't bind on ip:port. Error: {}", e);
            exit(-1);
        }
    };
    server.run(echo_protocol).await;
    Ok(())
}


// Handle the connection of the echo client, should be a struct with method lik
async fn echo_protocol(stream: TcpConnection) -> io::Result<()> {
    let stream = stream.stream;
    let peer_addr = stream.peer_addr()?;
    let (mut recv, mut send) = io::split(stream);
    io::copy(&mut recv, &mut send).await?;
    Ok(())
}
//
// struct EchoProtocol {
//     nbUser: usize,
//     connection: TcpConnection
// }
//
// impl EchoProtocol {
//     fn new(connection: TcpConnection) -> EchoProtocol {
//         EchoProtocol{nbUser: 0, connection}
//     }
// }
//
// #[async_trait]
// impl AppProtocol for EchoProtocol {
//     async fn data_received(&mut self) {
//         println!("Data received");
//     }
}