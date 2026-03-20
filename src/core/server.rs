use std::{
    io,
    sync::{Arc, Mutex},
};

use rosc::OscPacket;

use crate::{
    core::{Position, Velocity, manager::TuioManager, osc_sender::OscSender},
    tuio11::{
        Cursor,
        entity::{self, TuioEntity},
        manager::Manager,
    },
};

pub struct CursorHandle(Arc<Mutex<Cursor>>);

impl CursorHandle {
    pub fn set_position(&self, position: Position) {
        self.0.lock().unwrap().set_position(position);
    }

    pub fn position(&self) -> Position {
        self.0.lock().unwrap().position()
    }
}

pub struct Server<S: OscSender> {
    manager: Manager,
    sender: S,
}

impl<S: OscSender> Server<S> {
    pub fn new(sender: S, manager: Manager) -> Self {
        Self { manager, sender }
    }

    pub fn add_cursor(&mut self, position: Position) -> CursorHandle {
        let cursor = Cursor::new(
            self.manager.current_session_id(),
            position,
            Velocity::default(),
            0.0,
        );

        let handle = CursorHandle(Arc::new(Mutex::new(cursor)));
        // self.manager.add
        handle
    }

    pub fn remove(&mut self, entity: TuioEntity) {
        self.manager.remove(entity);
    }

    pub fn send_frame(&mut self) -> Result<(), io::Error> {
        // // for bundle in self.manager.update(entities) {
        //     self.sender.send(&OscPacket::Bundle(bundle.clone()))?;
        // }
        Ok(())
    }

    pub fn quit(&mut self) -> Result<(), io::Error> {
        for bundle in self.manager.quit() {
            self.sender.send(&OscPacket::Bundle(bundle.clone()))?;
        }
        Ok(())
    }

    pub fn next_session_id(&self) -> i32 {
        self.manager.current_session_id()
    }
}
