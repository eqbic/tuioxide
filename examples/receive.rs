use std::net::Ipv4Addr;

use tuioxide::tuio11::client::Client;

fn main() -> anyhow::Result<()> {
    let client = Client::new(Ipv4Addr::LOCALHOST, 3333)?;
    client.connect()?;
    loop {
        client.update()?;
    }
    Ok(())
}
