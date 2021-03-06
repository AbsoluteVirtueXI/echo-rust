/// Make the network great again!!

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::SystemTime;
use std::mem::size_of_val;
use std::future::Future;
use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use tokio::stream::{StreamExt};
use async_trait::async_trait;

const UNSPECIFIED_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
const UNSPECIFIED_PORT: u16 = 0;
const MAX_CONNECTION: usize = 100;

fn _get_local_SocketAddr(stream: &TcpStream) -> SocketAddr {
    match stream.local_addr() {
        Ok(socket) => socket,
        Err(_) => _get_unspecified_SocketAddr()
    }
}

fn _get_peer_SocketAddr(stream: &TcpStream) -> SocketAddr {
    match stream.peer_addr() {
        Ok(socket) => socket,
        Err(_) => _get_unspecified_SocketAddr()
    }
}

fn _get_unspecified_SocketAddr() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(UNSPECIFIED_IP, UNSPECIFIED_PORT))
}
/*
enum TransportLayer {
    Tcp,
    Udp,
}

pub enum Server {
    TcpServer(TcpServer),
    UdpServer(UdpServer),
}

impl Server {
    pub async fn new(transport: TransportLayer) -> io::Result<Server> {
        match transport {
            TransportLayer::Tcp => Server::TcpServer(TcpServer::new().await),
            TransportLayer::Udp => unimplemented!(),
        }
    }
}
*/
enum Client {
    TcpClient(TcpClient),
    UdpClient(UdpClient),
}

// TODO: add a connection pool to avoid DOS
pub struct TcpServer {
    // TODO: permit to 1 server to bind on multiple port
    pub listener: TcpListener,
    pub connection_pool: Vec<TcpConnection>,
}

impl TcpServer {
    pub async fn bind(s: &str) -> io::Result<TcpServer> {
        let res_server = TcpListener::bind("127.0.0.1:7777").await;
        let connection_pool = Vec::with_capacity(MAX_CONNECTION);
        match res_server {
            Ok(listener) => Ok(TcpServer { listener, connection_pool }),
            Err(e) => Err(e)
        }
    }

    pub async fn run<T>(&mut self, protocol: fn(TcpConnection) -> T)
        where T: Future + Send + 'static,
              T::Output: Send + 'static,
    {
        println!("Server started");
        while let Some(tcp_stream) = self.listener.incoming().map(
            |res_stream| {
                match res_stream {
                    Ok(stream) => Ok(TcpConnection::new(stream)),
                    Err(e) => Err(e), // TODO: maybe i can handle error here directl
                }
            }).next().await {
            match tcp_stream {
                Ok(connection) => {
                    self.handle_connection(protocol, connection);
                }
                Err(e) => {
                    println!("Connection Error: {}", e);
                }
            }
        }
    }

    fn handle_connection<T>(&mut self, protocol: fn(TcpConnection) -> T, connection: TcpConnection)
        where T: Future + Send + 'static,
              T::Output: Send + 'static
    {
        let peer = connection.peer_socket_addr;
        let local = connection.local_socket_addr;
        let date_open = connection.date_open;
        println!("Connection received from {} to {} at {:?}", peer, local, date_open);
        tokio::spawn(async move {
            protocol(connection).await;
            println!("Disconnection from {} to {}", peer, local);
        });
    }

    pub async fn stop() {}
}

// TODO: add a connection pool to avoid DDOS
struct TcpClient {}

impl TcpClient {
    pub async fn connect() {}
}

pub struct UdpServer {}

pub struct UdpClient {}

struct P2PClient {}

enum Connection {
    TCP(TcpConnection),
    UDP(UdpConnection),
}


// TODO: SystemTime new format ?
pub struct TcpConnection {
    pub stream: TcpStream,
    pub local_socket_addr: SocketAddr,
    pub peer_socket_addr: SocketAddr,
    pub date_open: SystemTime,
    pub date_close: Option<SystemTime>,
    pub tcp_stream: Vec<(Request, Response)>,
}

impl TcpConnection {
    pub fn new(stream: TcpStream) -> TcpConnection {
        let local_socket_addr = _get_local_SocketAddr(&stream);
        let peer_socket_addr = _get_peer_SocketAddr(&stream);
        let date_open = SystemTime::now();
        let date_close = None;
        let tcp_stream = Vec::new();
        TcpConnection {
            stream,
            local_socket_addr,
            peer_socket_addr,
            date_open,
            date_close,
            tcp_stream,
        }
    }

    // TODO: Should return a slice here
    pub fn local_as_str(&self) -> String {
        format!("{}:{}", self.local_socket_addr.ip(), self.local_socket_addr.port())
    }

    // TODO: Should return a slice here
    pub fn peer_as_str(&self) -> String {
        format!("{}:{}", self.peer_socket_addr.ip(), self.peer_socket_addr.port())
    }

    // TODO: should be in the server or client
    pub fn send(&mut self) {}

    // TODO: should be in the server or client
    pub fn recv(&mut self) {}
}

pub struct UdpConnection {}

// TODO: add src and dst
pub struct Request {
    pub date: SystemTime,
    pub content: Vec<u8>,
}

impl Request {
    pub fn from(buffer: &[u8]) -> Request {
        Request {
            date: SystemTime::now(),
            content: buffer.to_vec(),
        }
    }

    pub fn size(&self) -> usize {
        size_of_val(&self.content[..])
    }
}


// TODO: add a src dst
pub struct Response {
    pub date: SystemTime,
    pub content: Vec<u8>,
}

impl Response {
    pub fn from(buffer: &[u8]) -> Response {
        Response {
            date: SystemTime::now(),
            content: buffer.to_vec(),
        }
    }
    pub fn size(&self) -> usize {
        size_of_val(&self.content[..])
    }
}
//
// //TEST
// #[async_trait]
// pub trait AppProtocol {
//     async fn data_received(&mut self);
//
// }


