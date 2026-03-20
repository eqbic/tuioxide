use std::io;

use tuioxide::{
    core::{TuioTime, osc_sender::UdpOscSender, server::Server},
    tuio11::{Cursor, manager::Manager},
};

fn main() -> Result<(), io::Error> {
    let sender = UdpOscSender::default();
    let manager = Manager::new(&Some("Tuio Example".to_string()));
    let mut server = Server::new(sender, manager);
    let cursor = Cursor::new(manager, position, velocity, acceleration)
    Ok(())
}
