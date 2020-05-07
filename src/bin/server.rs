use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{StreamExt};
use std::process::exit;

use echo_rust::easy_net::*;


#[tokio::main]
async fn main() -> io::Result<()> {
    /// Bind listener to a port
    let mut listener = match TcpListener::bind("127.0.0.1:7777").await {
        Ok(listener) => {
            println!("Server running");
            listener
        },
        Err(e) => {
            println!("Server can't bind on ip:port. Error: {}", e);
            exit(-1);
        }
    };


    /// Loop over incoming connection
    while let Some(tcp_stream) = listener.incoming().map(|res_stream|{
        match res_stream {
            Ok(stream) => Ok(TcpConnection::new(stream)),
            Err(e) => Err(e)
        }
    }).next().await {
        match tcp_stream {
            Ok(connection) => {
                tokio::spawn(echo(connection));
            }
            Err(e) => {
                println!("Connection Error: {}", e);
            }
        }
    }
    Ok(())
}

/// Handle the connection of the echo client
async fn echo(stream: TcpConnection) -> io::Result<()> {
    let stream = stream.stream;
    let peer_addr = stream.peer_addr()?;
    println!("Connection from {}:{}", peer_addr.ip(), peer_addr.port());
    let (mut recv, mut send) = io::split(stream);
    io::copy(&mut recv, &mut send).await?;
    println!("Disconnection from {}:{}", peer_addr.ip(), peer_addr.port());
    Ok(())
}