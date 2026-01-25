// use std::{
//     cell::Cell,
//     net::Ipv4Addr,
//     sync::mpsc::{self, Receiver},
// };

// use rosc::OscPacket;

// use crate::{
//     common::{client::Client, osc_receiver::OscReceiver, tuio_time::TuioTime},
//     tuio20::osc_decoder::OscDecoder,
// };

// pub struct Processor<R>
// where
//     R: OscReceiver<OscPacket> + Send + Sync + 'static,
// {
//     client: Client<R>,
//     packet_receiver: Receiver<OscPacket>,
//     current_frame: Cell<i32>,
//     current_time: Cell<TuioTime>,
// }

// impl<R> Processor<R>
// where
//     R: OscReceiver<OscPacket> + Send + Sync + 'static,
// {
//     pub fn new(remote: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
//         let (sender, receiver) = mpsc::channel();
//         let client = Client::<R>::new(remote, port, sender)?;
//         Ok(Self {
//             current_frame: (-1).into(),
//             current_time: Cell::new(TuioTime::from_system_time()?),
//             client,
//             packet_receiver: receiver,
//         })
//     }

//     pub fn connect(&self) -> anyhow::Result<()> {
//         self.client.connect()?;
//         Ok(())
//     }

//     pub fn update(&self) -> anyhow::Result<()> {
//         let packet = self.packet_receiver.recv()?;
//         self.process_packet(packet)?;
//         Ok(())
//     }

//     fn process_packet(&self, packet: OscPacket) -> anyhow::Result<()> {
//         if let OscPacket::Bundle(bundle) = packet {
//             let tuio_bundle = OscDecoder::decode_bundle(bundle)?;
//             println!("{tuio_bundle:?}");
//         }
//         Ok(())
//     }
// }
