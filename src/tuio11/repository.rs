use std::collections::HashMap;

use rosc::OscBundle;

use crate::{core::TuioProfile, tuio11::osc_decoder_encoder::OscEncoder};

pub(crate) struct TuioRepository<P: TuioProfile> {
    source: String,
    entities: HashMap<i32, P>,
}

impl<P: TuioProfile> TuioRepository<P> {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
            entities: HashMap::new(),
        }
    }

    pub fn update(&mut self, frame_id: i32, entity: P) -> OscBundle {
        if let Some(e) = self.entities.get_mut(&entity.session_id()) {
            *e = entity
        }

        OscEncoder::encode_bundle(self.entities.values().cloned(), &self.source, frame_id)
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
