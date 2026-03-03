use std::io;

use tuioxide::tuio11::{Client, event::CursorEvent};

fn main() -> Result<(), io::Error> {
    let mut client = Client::default();
    loop {
        let events = client.update()?;
        for event in events.cursor_events {
            match event {
                CursorEvent::Add(cursor) => println!(
                    "New cursor [{}] at position {:?}",
                    cursor.session_id(),
                    cursor.position()
                ),
                CursorEvent::Update(cursor) => println!(
                    "Update cursor[{}] -> {:?}",
                    cursor.session_id(),
                    cursor.position()
                ),
                CursorEvent::Remove(cursor) => {
                    println!("Remove cursor[{}]", cursor.session_id())
                }
            }
        }
    }
}
