use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
};

use log::{debug, info};
use rosc::{OscPacket, decoder::MTU};

// #[derive(Debug)]
// pub struct WebsocketOscReceiver {
//     socket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>,
// }

// impl WebsocketOscReceiver {}

// impl OscReceiver<OscPacket> for WebsocketOscReceiver {
//     fn connect(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
//         let uri = format!("ws://{remote}:{port}").parse()?;
//         let builder = ClientRequestBuilder::new(uri);
//         let (socket, _) = connect(builder)?;
//         Ok(Self {
//             socket: socket.into(),
//         })
//     }

//     fn disconnect(&self) {
//         self.socket.lock().unwrap().close(None).unwrap()
//     }

//     fn is_connected(&self) -> bool {
//         self.socket.lock().unwrap().can_read()
//     }

//     fn recv(&self) -> anyhow::Result<OscPacket> {
//         let message = self.socket.lock().unwrap().read()?;
//         if let Message::Binary(data) = message {
//             let (_, packet) = rosc::decoder::decode_udp(&data)?;
//             Ok(packet)
//         } else {
//             Err(Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 "Could not decode OSC message.",
//             )
//             .into())
//         }
//     }
// }

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
