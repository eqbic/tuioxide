use std::net::Ipv4Addr;

use tuioxide::{
    client::{tuio11::processor::Processor, udp_receiver::UdpOscReceiver},
    core::tuio11::event::{CursorEvent, ObjectEvent},
};

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
        if let Some(tuio_events) = processor.update(packet) {
            for event in tuio_events.cursor_events {
                match event {
                    CursorEvent::Add(cursor) => println!("New cursor: {cursor:?}"),
                    CursorEvent::Update(cursor) => println!("Update cursor: {cursor:?}"),
                    CursorEvent::Remove(cursor) => println!("Remove cursor: {cursor:?}"),
                }
            }

            for event in tuio_events.object_events {
                match event {
                    ObjectEvent::Add(object) => println!("New object: {object:?}"),
                    ObjectEvent::Update(object) => println!("Update object: {object:?}"),
                    ObjectEvent::Remove(object) => println!("Remove object: {object:?}"),
                }
            }
        }
    }
}
