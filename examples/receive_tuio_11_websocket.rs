use std::net::Ipv4Addr;
use tuioxide::client::{tuio11::processor::Processor, websocket_receiver::WebsocketOscReceiver};

fn main() {
    let mut receiver = WebsocketOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
    let processor = Processor::default();
    loop {
        let packet = match receiver.recv() {
            Ok(packet) => packet,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        processor.update(packet);
        let cursors = processor.cursors();
        let objects = processor.objects();

        if !&cursors.is_empty() {
            println!("{cursors:?}");
        }

        if !&objects.is_empty() {
            eprintln!("{objects:?}");
        }
    }
}
