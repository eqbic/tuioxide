use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, TcpStream, UdpSocket},
    sync::{Arc, Mutex},
};

use rosc::OscPacket;
use tungstenite::{ClientRequestBuilder, Message, WebSocket, connect, stream::MaybeTlsStream};

pub trait OscReceiver<P>
where
    Self: Sized,
{
    fn connect(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self>;
    fn disconnect(&self);
    fn is_connected(&self) -> bool;
    fn recv(&self) -> anyhow::Result<P>;
}

#[derive(Debug)]
pub struct WebsocketOscReceiver {
    socket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl WebsocketOscReceiver {}

impl OscReceiver<OscPacket> for WebsocketOscReceiver {
    fn connect(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        let uri = format!("ws://{remote}:{port}").parse()?;
        let builder = ClientRequestBuilder::new(uri);
        let (socket, _) = connect(builder)?;
        Ok(Self {
            socket: socket.into(),
        })
    }

    fn disconnect(&self) {
        self.socket.lock().unwrap().close(None).unwrap()
    }

    fn is_connected(&self) -> bool {
        self.socket.lock().unwrap().can_read()
    }

    fn recv(&self) -> anyhow::Result<OscPacket> {
        let message = self.socket.lock().unwrap().read()?;
        if let Message::Binary(data) = message {
            let (_, packet) = rosc::decoder::decode_udp(&data)?;
            Ok(packet)
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not decode OSC message.",
            )
            .into())
        }
    }
}

#[derive(Debug)]
pub struct UdpOscReceiver {
    socket: Arc<UdpSocket>,
}

impl OscReceiver<OscPacket> for UdpOscReceiver {
    fn connect(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            socket: Arc::new(UdpSocket::bind(SocketAddrV4::new(remote, port))?),
        })
    }

    fn disconnect(&self) {}

    fn is_connected(&self) -> bool {
        true
    }

    fn recv(&self) -> anyhow::Result<OscPacket> {
        let mut buffer = [0u8; rosc::decoder::MTU];
        let size = self.socket.recv(&mut buffer)?;
        let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])?;
        Ok(packet)
    }
}
