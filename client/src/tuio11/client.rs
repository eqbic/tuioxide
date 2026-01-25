use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::Arc,
};

use rosc::{
    OscPacket,
    decoder::{MTU, decode_udp},
};

pub trait OscReceiver
where
    Self: Sized,
{
    fn connect(remote: Ipv4Addr, port: u16) -> Result<Self, Error>;
    fn disconnect(&self);
    fn is_connected(&self) -> bool;
    fn recv(&mut self) -> Result<OscPacket, Error>;
}

pub struct UdpReceiver {
    socket: Arc<UdpSocket>,
    is_connected: bool,
    buffer: [u8; MTU],
}

impl OscReceiver for UdpReceiver {
    fn connect(remote: Ipv4Addr, port: u16) -> Result<Self, Error> {
        let socket = UdpSocket::bind(SocketAddrV4::new(remote, port))?;
        Ok(Self {
            socket: Arc::new(socket),
            is_connected: true,
            buffer: [0u8; MTU],
        })
    }

    fn disconnect(&self) {}

    fn is_connected(&self) -> bool {
        self.is_connected
    }

    fn recv(&mut self) -> Result<OscPacket, Error> {
        let size = self.socket.recv(&mut self.buffer)?;
        let (_, packet) = decode_udp(&self.buffer[..size]).unwrap();
        Ok(packet)
    }
}

pub struct Client<R>
where
    R: OscReceiver,
{
    receiver: R,
}

impl<R> Client<R>
where
    R: OscReceiver,
{
    pub fn connect(remote: Ipv4Addr, port: u16) -> Result<Self, Error> {
        let receiver = R::connect(remote, port)?;
        Ok(Self { receiver })
    }

    pub fn update(&mut self) {
        let packet = self.receiver.recv().unwrap();
        println!("Packet: {:?}", packet);
    }
}
