// use std::{
//     net::Ipv4Addr,
//     sync::{Arc, mpsc::Sender},
//     thread,
// };

// use rosc::OscPacket;

// use crate::common::osc_receiver::OscReceiver;

// pub struct Client<T>
// where
//     T: OscReceiver<OscPacket> + Send + Sync + 'static,
// {
//     receiver: Arc<T>,
//     packet_sender: Sender<OscPacket>,
// }

// impl<T> Client<T>
// where
//     T: OscReceiver<OscPacket> + Send + Sync + 'static,
// {
//     pub fn new(
//         remote: Ipv4Addr,
//         port: u16,
//         packet_sender: Sender<OscPacket>,
//     ) -> anyhow::Result<Self> {
//         Ok(Self {
//             receiver: Arc::new(T::connect(remote, port)?),
//             packet_sender,
//         })
//     }

//     pub fn connect(&self) -> anyhow::Result<()> {
//         let receiver = self.receiver.clone();
//         let sender = self.packet_sender.clone();
//         thread::spawn(move || {
//             while let Ok(packet) = receiver.recv() {
//                 sender.send(packet).unwrap();
//             }
//         });
//         Ok(())
//     }

//     pub fn disconnect(&self) {
//         self.receiver.disconnect()
//     }
// }
