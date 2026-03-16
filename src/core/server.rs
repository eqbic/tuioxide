use crate::core::osc_sender::OscSender;

pub struct Server {
    sender: Box<dyn OscSender>,
}

impl Server {
    pub fn update(&self) {
        
    }
}
