use std::net::Ipv4Addr;

use tuioxide::client::common::osc_receiver::UdpOscReceiver;

fn main() {
    let mut receiver = match UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333) {
        Ok(receiver) => receiver,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
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
            println!("{objects:?}");
        }
    }
}
