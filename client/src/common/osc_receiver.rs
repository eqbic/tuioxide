use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, TcpStream, UdpSocket},
};

use log::{debug, info};
use rosc::{OscPacket, decoder::MTU};
use tungstenite::{ClientRequestBuilder, Message, WebSocket, connect, stream::MaybeTlsStream};

#[derive(Debug)]
pub struct WebsocketOscReceiver {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl WebsocketOscReceiver {}

impl WebsocketOscReceiver {
    pub fn new(remote: Ipv4Addr, port: u16) -> Self {
        let uri = format!("ws://{remote}:{port}").parse().unwrap();
        let builder = ClientRequestBuilder::new(uri);
        let (socket, _) = connect(builder).unwrap();
        Self { socket }
    }

    pub fn recv(&mut self) -> Result<OscPacket, io::Error> {
        let message = self.socket.read().unwrap();
        if let Message::Binary(data) = message {
            let (_, packet) = rosc::decoder::decode_udp(&data).unwrap();
            Ok(packet)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))
        }
    }
}

#[derive(Debug)]
pub struct UdpOscReceiver {
    socket: UdpSocket,
    buffer: [u8; MTU],
}

impl UdpOscReceiver {
    pub fn new(remote: Ipv4Addr, port: u16) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(SocketAddrV4::new(remote, port))?;
        info!("Created UDP socket for {remote}:{port}");
        Ok(Self {
            socket,
            buffer: [0u8; MTU],
        })
    }

    pub fn recv(&mut self) -> Result<OscPacket, io::Error> {
        let size = self.socket.recv(&mut self.buffer)?;
        let (_, packet) = rosc::decoder::decode_udp(&self.buffer[..size]).unwrap();
        debug!("{:?}", packet);
        Ok(packet)
    }
}
