use std::{
    cell::RefCell,
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

pub struct WebsocketReceiver {
    socket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl WebsocketReceiver {}

impl OscReceiver<OscPacket> for WebsocketReceiver {
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

pub struct UdpReceiver {
    remote: Ipv4Addr,
    port: u16,
    socket: Arc<UdpSocket>,
}

impl OscReceiver<OscPacket> for UdpReceiver {
    fn connect(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            remote,
            port,
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
