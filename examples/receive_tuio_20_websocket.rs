use std::io;

use tuioxide::{
    core::WebsocketOscReceiver,
    tuio20::{Client, PointerEvent, TokenEvent},
};

fn main() -> Result<(), io::Error> {
    let mut client = Client::new(WebsocketOscReceiver::default());

    loop {
        let events = client.update()?;

        println!("Frame: {:?}", events.frame_event);

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

        for event in events.token_events {
            match event {
                TokenEvent::Add(token) => println!(
                    "New token [{}] with id {} at position {:?}",
                    token.session_id(),
                    token.component_id(),
                    token.position()
                ),
                TokenEvent::Update(token) => println!(
                    "Update token[{}] with id {} -> {:?}",
                    token.session_id(),
                    token.component_id(),
                    token.position()
                ),
                TokenEvent::Remove(token) => {
                    println!("Remove token[{}]", token.session_id())
                }
            }
        }
    }
}
