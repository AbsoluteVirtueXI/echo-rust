use tokio::io;
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:7777").await?;
    loop {
        let (stream, socket) = listener.accept().await?;
        println!("Connection from {}:{}", socket.ip(), socket.port());
        echo(stream).await?;
        println!("Disconnection from {}:{}", socket.ip(), socket.port());
    }
}

async fn echo(stream: TcpStream) -> io::Result<()> {
    let (mut recv, mut send) = io::split(stream);
    io::copy(&mut recv, &mut send).await?;
    Ok(())
}