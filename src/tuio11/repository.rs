use std::collections::HashMap;

use rosc::OscBundle;

use crate::{core::TuioEntity, tuio11::osc_decoder_encoder::OscEncoder};

pub(crate) struct TuioRepository<E: TuioEntity> {
    source: Option<String>,
    entities: HashMap<i32, E>,
    tuio_address: String,
    frame_id: i32,
}

impl<E: TuioEntity> TuioRepository<E> {
    pub fn new(source: &Option<String>) -> Self {
        Self {
            source: source.clone(),
            entities: HashMap::new(),
            tuio_address: E::address(),
            frame_id: 0,
        }
    }

    pub fn update(&mut self, frame_id: i32) {
        self.frame_id = frame_id
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
