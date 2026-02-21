use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
};

use log::{debug, info};
use rosc::{OscPacket, decoder::MTU};

pub trait OscReceiver: Default {
    fn new(remote: Ipv4Addr, port: u16) -> Self;
    fn recv(&mut self) -> Result<OscPacket, io::Error>;
}

#[derive(Debug)]
pub struct UdpOscReceiver {
    socket: UdpSocket,
    buffer: [u8; MTU],
}

impl OscReceiver for UdpOscReceiver {
    fn new(remote: Ipv4Addr, port: u16) -> Self {
        let socket = match UdpSocket::bind(SocketAddrV4::new(remote, port)) {
            Ok(socket) => {
                info!("Created UDP socket for {remote}:{port}");
                socket
            }
            Err(error) => panic!("Could not bind to socket: {}", error),
        };

        Self {
            socket,
            buffer: [0u8; MTU],
        }
    }

    fn recv(&mut self) -> Result<OscPacket, io::Error> {
        let size = self.socket.recv(&mut self.buffer)?;
        let (_, packet) = rosc::decoder::decode_udp(&self.buffer[..size]).unwrap();
        debug!("{:?}", packet);
        Ok(packet)
    }
}

impl Default for UdpOscReceiver {
    fn default() -> Self {
        Self::new(Ipv4Addr::LOCALHOST, 3333)
    }
}

#[cfg(feature = "websocket")]
pub mod websocket {
    use rosc::OscPacket;
    use std::{
        io,
        net::{Ipv4Addr, TcpStream},
    };
    use tungstenite::{ClientRequestBuilder, Message, WebSocket, connect, stream::MaybeTlsStream};

    use crate::client::osc_receiver::OscReceiver;

    #[derive(Debug)]
    pub struct WebsocketOscReceiver {
        socket: WebSocket<MaybeTlsStream<TcpStream>>,
    }

    impl OscReceiver for WebsocketOscReceiver {
        fn new(remote: Ipv4Addr, port: u16) -> Self {
            let uri = format!("ws://{remote}:{port}").parse().unwrap();
            let builder = ClientRequestBuilder::new(uri);
            let (socket, _) = connect(builder).unwrap();
            Self { socket }
        }

        fn recv(&mut self) -> Result<OscPacket, io::Error> {
            let message = self.socket.read().unwrap();
            if let Message::Binary(data) = message {
                let (_, packet) = rosc::decoder::decode_udp(&data).unwrap();
                Ok(packet)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))
            }
        }
    }

    impl Default for WebsocketOscReceiver {
        fn default() -> Self {
            Self::new(Ipv4Addr::LOCALHOST, 3333)
        }
    }
}
