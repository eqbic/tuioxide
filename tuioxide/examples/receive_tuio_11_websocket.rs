use client::{common::osc_receiver::WebsocketOscReceiver, tuio11::processor::Processor};
use log::{error, info};
use std::net::Ipv4Addr;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut receiver = WebsocketOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
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
        let cursors = processor.cursors();
        let objects = processor.objects();

        if !&cursors.is_empty() {
            info!("{cursors:?}");
        }

        if !&objects.is_empty() {
            info!("{objects:?}");
        }
    }
}
