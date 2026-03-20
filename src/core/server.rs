use std::{cell::RefCell, rc::Rc};

use rosc::OscPacket;

use crate::core::{manager::TuioManager, osc_sender::OscSender};

pub struct Server<M: TuioManager + 'static> {
    sender: Box<dyn OscSender>,
    manager: Rc<M>,
}

impl<M: TuioManager> Server<M> {
    pub fn new(sender: impl OscSender + 'static, manager: M) -> Self {
        Self {
            sender: Box::new(sender),
            manager: Rc::new(manager),
        }
    }

    pub fn send(&mut self) {
        // let bundles = Rc::make_mut(&mut self.manager).update();
        // for bundle in bundles {
        //     if let Err(error) = self.sender.send(&OscPacket::Bundle(bundle.clone())) {
        //         log::error!("Could not send Osc Bundle: {error}");
        //     }
        // }
    }
}
