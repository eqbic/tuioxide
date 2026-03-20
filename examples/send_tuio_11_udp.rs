use std::io;

use tuioxide::{
    core::{Position, osc_sender::UdpOscSender},
    tuio11::server::Server,
};

fn main() -> Result<(), io::Error> {
    let sender = UdpOscSender::default();
    let mut server = Server::new("Test Source");
    server.add_sender(sender);

    let position_delta = Position::new(0.1, 0.2);

    let mut cursor = server.add_cursor(server.next_session_id(), Position::default());
    let mut cursor_2 = server.add_cursor(server.next_session_id(), Position::new(0.5, 0.1));
    let mut object = server.add_object(server.next_session_id(), 4, Position::default(), 0.0);
    for _ in 0..10 {
        cursor.set_position(cursor.position() + position_delta);
        server.update_cursor(cursor);
        server.send_frame()?;
    }
    server.quit()?;
    println!("test");
    Ok(())
}
