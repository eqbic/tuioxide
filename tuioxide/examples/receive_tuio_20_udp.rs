use std::net::Ipv4Addr;

use tuioxide::client::{osc_receiver::UdpOscReceiver, tuio20::processor::Processor};

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
        let pointers = processor.pointers();
        let tokens = processor.tokens();
        if !&pointers.is_empty() {
            println!("{pointers:?}");
        }

        if !&tokens.is_empty() {
            println!("{tokens:?}");
        }
    }
}
