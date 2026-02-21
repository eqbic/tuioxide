use std::io;

use crate::client::{
    osc_receiver::{OscReceiver, UdpOscReceiver},
    tuio20::processor::Processor,
};

pub struct Client<T: OscReceiver> {
    receiver: T,
    processor: Processor,
}

impl<T> Client<T>
where
    T: OscReceiver,
{
    pub fn new(receiver: T) -> Self {
        Self {
            receiver,
            processor: Processor::default(),
        }
    }

    pub fn update(&mut self) -> Result<TuioEvents, io::Error> {
        let packet = self.receiver.recv()?;
        self.processor.update(packet).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "No valid Tuio Bundle",
        ))
    }
}

impl Default for Client<UdpOscReceiver> {
    fn default() -> Self {
        Self::new(UdpOscReceiver::default())
    }
}
