use std::net::Ipv4Addr;

use tuioxide::{
    common::osc_receiver::{UdpOscReceiver, WebsocketOscReceiver},
    tuio11::client::Client,
};

fn main() -> anyhow::Result<()> {
    let client = Client::<UdpOscReceiver>::new(Ipv4Addr::LOCALHOST, 3333)?;
    client.connect()?;
    loop {
        client.update()?;
        let cursors = client.cursors();
        let objects = client.objects();
        if !&cursors.is_empty() {
            println!("{:?}", cursors);
        }

        if !&objects.is_empty() {
            println!("{:?}", objects);
        }
    }
    Ok(())
}
