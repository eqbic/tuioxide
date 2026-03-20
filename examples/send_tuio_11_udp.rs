use std::io;

use tuioxide::{
    core::{Position, Velocity, osc_sender::UdpOscSender, server::Server},
    tuio11::{Cursor, entity::TuioEntity, manager::Manager},
};

fn main() -> Result<(), io::Error> {
    let manager = Manager::new(&Some("Tuio Example".to_string()));
    let sender = UdpOscSender::default();
    let mut server = Server::new(sender, manager);

    let position_delta = Position::new(0.1, 0.2);
    let mut cursor = Cursor::new(
        server.next_session_id(),
        Position::default(),
        Velocity::default(),
        0.0,
    );
    let cursor = server.add_cursor(Position::default());
    for _ in 0..100 {
        cursor.set_position(cursor.position() + position_delta);
        server.send_frame()?;
    }
    server.quit()?;
    println!("test");
    Ok(())
}
