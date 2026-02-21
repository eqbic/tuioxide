use rosc::OscPacket;
use std::{
    io,
    net::{Ipv4Addr, TcpStream},
};

use tungstenite::{ClientRequestBuilder, Message, WebSocket, connect, stream::MaybeTlsStream};

#[derive(Debug)]
pub struct WebsocketOscReceiver {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

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
