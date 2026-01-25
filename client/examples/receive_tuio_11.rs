// use std::net::Ipv4Addr;

// use tuioxide::{common::osc_receiver::UdpOscReceiver, tuio11::processor::Processor};

// fn main() -> anyhow::Result<()> {
//     let tuio11_processor = Processor::<UdpOscReceiver>::new(Ipv4Addr::LOCALHOST, 3333)?;
//     tuio11_processor.connect()?;
//     loop {
//         tuio11_processor.update()?;
//         let cursors = tuio11_processor.cursors();
//         let objects = tuio11_processor.objects();
//         if !&cursors.is_empty() {
//             println!("{cursors:?}");
//         }

//         if !&objects.is_empty() {
//             println!("{objects:?}");
//         }
//     }
// }

use std::net::Ipv4Addr;

use client::tuio11::client::{Client, UdpReceiver};

fn main() {
    let mut client = Client::<UdpReceiver>::connect(Ipv4Addr::LOCALHOST, 3333).unwrap();
    loop {
        client.update();
    }
}
