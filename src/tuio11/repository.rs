use rosc::{OscMessage, OscType};

use crate::core::Profile;

struct TuioRepository {
    source: Option<String>,
    tuio_address: String,
    frame_id: u32,
}

impl TuioRepository {
    pub fn new(source: Option<String>, tuio_address: String) -> Self {
        Self {
            source,
            tuio_address,
            frame_id: 0,
        }
    }

    pub fn source_message(&self) -> Option<OscMessage> {
        if let Some(source) = &self.source {
            let message = OscMessage {
                addr: self.tuio_address.clone(),
                args: vec![
                    OscType::String("source".into()),
                    OscType::String(source.into()),
                ],
            };
            return Some(message);
        }
        None
    }
}
