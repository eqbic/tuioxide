use std::collections::HashMap;

use rosc::OscBundle;

use crate::{core::TuioEntity, tuio11::osc_decoder_encoder::OscEncoder};

struct TuioRepository<E: TuioEntity> {
    source: Option<String>,
    entities: HashMap<i32, E>,
    tuio_address: String,
    frame_id: i32,
}

impl<E: TuioEntity> TuioRepository<E> {
    pub fn new(source: Option<String>, tuio_address: String) -> Self {
        Self {
            source,
            entities: HashMap::new(),
            tuio_address,
            frame_id: 0,
        }
    }

    pub fn add(&mut self, entity: E) {
        self.entities.insert(entity.session_id(), entity);
    }

    pub fn remove(&mut self, session_id: i32) {
        self.entities.remove(&session_id);
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn bundle(&self) -> OscBundle {
        OscEncoder::encode_bundle(
            self.entities.values().cloned(),
            self.source.as_deref(),
            self.frame_id,
        )
    }
}
