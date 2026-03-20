use std::{
    hash::Hash,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

use rosc::OscPacket;

pub trait OscSender: Send + Hash + Eq {
    fn send(&self, packet: &OscPacket) -> Result<(), io::Error>;
}

#[derive(Debug)]
pub struct UdpOscSender {
    socket: UdpSocket,
    address: SocketAddr,
}

impl UdpOscSender {
    pub fn new(target: SocketAddr) -> Result<Self, io::Error> {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let socket = UdpSocket::bind(bind_addr)?;
        Ok(Self {
            socket,
            address: target,
        })
    }
}

impl Eq for UdpOscSender {}

impl PartialEq for UdpOscSender {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Hash for UdpOscSender {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
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
