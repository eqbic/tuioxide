use std::net::Ipv4Addr;

use tuioxide::{common::osc_receiver::WebsocketOscReceiver, tuio11::client::Client};

fn main() -> anyhow::Result<()> {
    let client = Client::<WebsocketOscReceiver>::new(Ipv4Addr::LOCALHOST, 3333)?;
    client.connect()?;
    loop {
        client.update()?;
    }
    Ok(())
}
