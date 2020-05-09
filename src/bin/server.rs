use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{StreamExt};
use std::process::exit;

use echo_rust::easy_net::*;


/// main
#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind listener to a port
    let mut server = match TcpServer::bind("127.0.0.1:7777").await {
        Ok(server) => {
            println!("Server running");
            server
        },
        Err(e) => {
            println!("Server can't bind on ip:port. Error: {}", e);
            exit(-1);
        }
    };

    // Loop over incoming connection
    server.run(echo_protocol).await;

    Ok(())
}


/// Handle the connection of the echo client
async fn echo_protocol(stream: TcpConnection) -> io::Result<()> {
    let stream = stream.stream;
    let peer_addr = stream.peer_addr()?;
    println!("Connection from {}:{}", peer_addr.ip(), peer_addr.port());
    let (mut recv, mut send) = io::split(stream);
    io::copy(&mut recv, &mut send).await?;
    println!("Disconnection from {}:{}", peer_addr.ip(), peer_addr.port());
    Ok(())
}