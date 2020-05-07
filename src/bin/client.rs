use tokio::net::TcpStream;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()>  {
    let mut stream = TcpStream::connect("127.0.0.1:7777").await?;
    let (mut recv, mut send) = io::split(stream);
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let sendx = tokio::spawn ( async move {
        io::copy(&mut stdin, &mut send).await
    });
    let recvx = tokio::spawn( async move {
        io::copy(&mut recv, &mut stdout).await
    });
    sendx.await?;
    //recvx.await?;
    println!("END OF PROGRAM CLIENT.");
    Ok(())
}