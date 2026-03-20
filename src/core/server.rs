use std::io;

use rosc::OscPacket;

use crate::{
    core::{manager::TuioManager, osc_sender::OscSender},
    tuio11::{
        entity::{self, TuioEntity},
        manager::Manager,
    },
};

pub struct Server<S: OscSender> {
    manager: Manager,
    sender: S,
}

impl<S: OscSender> Server<S> {
    pub fn new(sender: S, manager: Manager) -> Self {
        Self { manager, sender }
    }

    pub fn add(&mut self, entity: TuioEntity) {
        self.manager.add(entity);
    }

    pub fn remove(&mut self, entity: TuioEntity) {
        self.manager.remove(entity);
    }

    pub fn send_frame(&mut self, entities: &[TuioEntity]) -> Result<(), io::Error> {
        for bundle in self.manager.update(entities) {
            self.sender.send(&OscPacket::Bundle(bundle.clone()))?;
        }
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
