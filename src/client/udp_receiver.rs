use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
};

use log::{debug, info};
use rosc::{OscPacket, decoder::MTU};

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
