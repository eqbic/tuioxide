use std::io;

use rosc::OscPacket;
use tuioxide::{
    core::{
        Position, Velocity,
        manager::TuioManager,
        osc_sender::{OscSender, UdpOscSender},
    },
    tuio11::{Cursor, entity::TuioEntity, manager::Manager},
};

fn main() -> Result<(), io::Error> {
    let mut manager = Manager::new(&Some("Tuio Example".to_string()));
    let sender = UdpOscSender::default();
    let position_delta = Position::new(0.1, 0.2);
    let mut cursor = Cursor::new(
        manager.current_session_id(),
        Position::default(),
        Velocity::default(),
        0.0,
    );

    manager.add(TuioEntity::Cursor(cursor));

    loop {
        cursor.set_position(cursor.position() + position_delta);
        for bundle in manager.update(&[TuioEntity::Cursor(cursor)]) {
            sender.send(&OscPacket::Bundle(bundle.clone()))?;
        }
    }
    Ok(())
}
