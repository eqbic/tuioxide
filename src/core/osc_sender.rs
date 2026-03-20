use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

use rosc::OscPacket;

pub trait OscSender: Send {
    fn send(&self, packet: &OscPacket) -> Result<(), io::Error>;
}

pub struct UdpOscSender {
    socket: UdpSocket,
    address: SocketAddr,
}

impl UdpOscSender {
    pub fn new(target: SocketAddr) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(target)?;
        Ok(Self {
            socket,
            address: target,
        })
    }
}

impl OscSender for UdpOscSender {
    fn send(&self, packet: &OscPacket) -> Result<(), io::Error> {
        let buffer = rosc::encoder::encode(packet).unwrap();
        if let Err(error) = self.socket.send_to(&buffer, self.address) {
            log::error!("Could not send osc packet to {}: {}", self.address, error);
        }
        Ok(())
    }
}

impl Default for UdpOscSender {
    fn default() -> Self {
        Self::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3333))
            .expect("Could not create UdpOscSender.")
    }
}
