use rosc::OscPacket;

use crate::core::{manager::TuioManager, osc_sender::OscSender};

pub struct Server<M: TuioManager> {
    sender: Box<dyn OscSender>,
    manager: M,
}

impl<M: TuioManager> Server<M> {
    pub fn new(sender: impl OscSender + 'static, manager: M) -> Self {
        Self {
            sender: Box::new(sender),
            manager,
        }
    }

    pub fn send(&mut self) {
        let bundles = self.manager.update();
        for bundle in bundles {
            if let Err(error) = self.sender.send(&OscPacket::Bundle(bundle.clone())) {
                log::error!("Could not send Osc Bundle: {error}");
            }
        }
    }
}
