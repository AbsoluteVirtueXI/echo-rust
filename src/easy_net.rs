/// Make the network great again!!

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::SystemTime;
use std::mem::size_of_val;
use tokio::net::{TcpListener, TcpStream};


const UNSPECIFIED_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
const UNSPECIFIED_PORT: u16 = 0;

fn _get_local_SocketAddr(stream: &TcpStream) -> SocketAddr {
    match stream.local_addr() {
        Ok(socket) => socket,
        Err(e) => _get_unspecified_SocketAddr()
    }
}

fn _get_peer_SocketAddr(stream: &TcpStream) -> SocketAddr {
    match stream.peer_addr() {
        Ok(socket) => socket,
        Err(e) => _get_unspecified_SocketAddr()
    }
}

fn _get_unspecified_SocketAddr() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(UNSPECIFIED_IP, UNSPECIFIED_PORT))
}

/*
pub struct TcpServer {
    server : TcpListener,
}

impl TcpServer {
    pub async fn bind() {}
    pub async fn start() {}
    pub async fn stop() {}
}

struct TcpClient {
}

impl TcpClient {
    pub async fn connect() {}
}
*/

pub struct TcpConnection {
    pub stream: TcpStream,
    pub local_SocketAddr: SocketAddr,
    pub peer_SocketAddr: SocketAddr,
    pub date_open: SystemTime,
    pub date_close: Option<SystemTime>,
    pub tcp_stream: Vec<(Request, Response)>,
}

impl TcpConnection {
    pub fn new(stream: TcpStream) -> TcpConnection {
        let local_SocketAddr = _get_local_SocketAddr(&stream);
        let peer_SocketAddr = _get_peer_SocketAddr(&stream);
        let date_open = SystemTime::now();
        let date_close = None;
        let tcp_stream = Vec::new();
        TcpConnection {
            stream,
            local_SocketAddr,
            peer_SocketAddr,
            date_open,
            date_close,
            tcp_stream,
        }
    }

    // TODO: Should return a slice here
    pub fn local_as_str(&self) -> String {
        format!("{}:{}", self.local_SocketAddr.ip(), self.local_SocketAddr.port())
    }

    // TODO: Should return a slice here
    pub fn peer_as_str(&self) -> String {
        format!("{}:{}", self.peer_SocketAddr.ip(), self.peer_SocketAddr.port())
    }

}


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