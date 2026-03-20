use std::io;

use tuioxide::{
    core::{
        Position, TuioTime, Velocity, manager::TuioManager, osc_sender::UdpOscSender,
        server::Server,
    },
    tuio11::{Cursor, manager::Manager},
};

fn main() -> Result<(), io::Error> {
    let sender = UdpOscSender::default();
    let manager = Manager::new(&Some("Tuio Example".to_string()));
    let mut server = Server::new(sender, manager);
    let cursor = Cursor::new(
        manager.current_session_id(),
        Position::default(),
        Velocity::default(),
        0.0,
    );
    Ok(())
}
