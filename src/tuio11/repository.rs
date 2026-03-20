use std::collections::HashMap;

use rosc::OscBundle;

use crate::{core::TuioProfile, tuio11::osc_decoder_encoder::OscEncoder};

pub(crate) struct TuioRepository<P: TuioProfile> {
    source: Option<String>,
    entities: HashMap<i32, P>,
    tuio_address: String,
}

impl<P: TuioProfile> TuioRepository<P> {
    pub fn new(source: &Option<String>) -> Self {
        Self {
            source: source.clone(),
            entities: HashMap::new(),
            tuio_address: P::address(),
        }
    }

    pub fn update(&mut self, frame_id: i32) -> OscBundle {
        OscEncoder::encode_bundle(
            self.entities.values().cloned(),
            self.source.as_deref(),
            frame_id,
        )
    }

    pub fn add(&mut self, entity: P) {
        self.entities.insert(entity.session_id(), entity);
    }

    pub fn remove(&mut self, session_id: i32) {
        self.entities.remove(&session_id);
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }
}
