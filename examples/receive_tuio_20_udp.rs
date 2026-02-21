use std::io;

use tuioxide::{client::tuio20::client::Client, core::tuio20::events::PointerEvent};

fn main() -> Result<(), io::Error> {
    let mut client = Client::default();

    loop {
        let events = client.update()?;
        for event in events.pointer_events {
            match event {
                PointerEvent::Add(pointer) => println!(
                    "New pointer [{}] at position {:?}",
                    pointer.session_id(),
                    pointer.position()
                ),
                PointerEvent::Update(pointer) => println!(
                    "Update pointer[{}] -> {:?}",
                    pointer.session_id(),
                    pointer.position()
                ),
                PointerEvent::Remove(pointer) => {
                    println!("Remove pointer[{}]", pointer.session_id())
                }
            }
        }
    }
}
