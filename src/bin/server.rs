use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{StreamExt};

use std::process::exit;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = match TcpListener::bind("127.0.0.1:7777").await {
        Ok(listener) => listener,
        Err(e) => {
            println!("Server can't bind on ip:port. Error: {}", e);
            exit(-1);
        }
    };

    while let Some(tcp_stream) = listener.incoming().next().await {
        match tcp_stream {
            Ok(connection) => {
                let peer_addr = connection.peer_addr()?;
                println!("Connection from {}:{}", peer_addr.ip(), peer_addr.port());
                echo(connection).await?;
                println!("Disconnection from {}:{}", peer_addr.ip(), peer_addr.port());
            }
            Err(e) => {
                println!("Connection Error: {}", e);
            }
        }
    }

    Ok(())

}

async fn echo(stream: TcpStream) -> io::Result<()> {
    let (mut recv, mut send) = io::split(stream);
    io::copy(&mut recv, &mut send).await?;
    Ok(())
}