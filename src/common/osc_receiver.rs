use std::{
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::Arc,
};

use rosc::OscPacket;

pub trait OscReceiver<P> {
    fn connect(&self) -> anyhow::Result<()>;
    fn disconnect(&self) -> anyhow::Result<()>;
    fn is_connected(&self) -> bool;
    fn recv(&self) -> anyhow::Result<P>;
}

pub struct UdpReceiver {
    remote: Ipv4Addr,
    port: u16,
    socket: Arc<UdpSocket>,
}

impl UdpReceiver {
    pub fn new(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            remote,
            port,
            socket: Arc::new(UdpSocket::bind(SocketAddrV4::new(remote, port))?),
        })
    }
}

impl OscReceiver<OscPacket> for UdpReceiver {
    fn connect(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn disconnect(&self) -> anyhow::Result<()> {
        Ok(())
    }

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
