use client::{common::osc_receiver::UdpOscReceiver, tuio20::processor::Processor};
use log::error;
use std::net::Ipv4Addr;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut receiver = match UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333) {
        Ok(receiver) => receiver,
        Err(error) => {
            error!("{error}");
            return;
        }
    };

    let processor = Processor::default();
    loop {
        let packet = match receiver.recv() {
            Ok(packet) => packet,
            Err(error) => {
                error!("{error}");
                continue;
            }
        };
        processor.update(packet);
    }
}
