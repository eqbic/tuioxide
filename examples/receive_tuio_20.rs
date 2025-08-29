use std::net::Ipv4Addr;

use tuioxide::{common::osc_receiver::WebsocketOscReceiver, tuio20::processor::Processor};

fn main() -> anyhow::Result<()> {
    let processor = Processor::<WebsocketOscReceiver>::new(Ipv4Addr::LOCALHOST, 3333)?;
    processor.connect()?;
    loop {
        processor.update()?;
    }
}
