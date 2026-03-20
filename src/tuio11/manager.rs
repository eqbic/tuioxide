use rosc::OscBundle;

use crate::{
    core::manager::TuioManager,
    tuio11::{Blob, Cursor, Object, entity::TuioEntity, repository::TuioRepository},
};

pub struct Manager {
    cursors: TuioRepository<Cursor>,
    objects: TuioRepository<Object>,
    blobs: TuioRepository<Blob>,
    frame_bundles: Vec<OscBundle>,
    current_session_id: i32,
    frame_id: i32,
}

impl Manager {
    pub fn new(source: &Option<String>) -> Self {
        Self {
            cursors: TuioRepository::new(source),
            objects: TuioRepository::new(source),
            blobs: TuioRepository::new(source),
            frame_bundles: Vec::with_capacity(3),
            current_session_id: 0,
            frame_id: 0,
        }
    }
}
impl TuioManager for Manager {
    type TuioEntity = TuioEntity;
    fn update(&mut self, entities: &[TuioEntity]) -> &Vec<OscBundle> {
        self.frame_id += 1;
        self.frame_bundles.clear();
        for entity in entities {
            match entity {
                TuioEntity::Cursor(cursor) => {
                    self.frame_bundles
                        .push(self.cursors.update(self.frame_id, *cursor));
                }
                TuioEntity::Object(object) => {
                    self.frame_bundles
                        .push(self.objects.update(self.frame_id, *object));
                }
                TuioEntity::Blob(blob) => {
                    self.frame_bundles
                        .push(self.blobs.update(self.frame_id, *blob));
                }
            }
        }
        &self.frame_bundles
    }

    fn add(&mut self, entity: TuioEntity) {
        match entity {
            TuioEntity::Cursor(cursor) => self.cursors.add(cursor),
            TuioEntity::Object(object) => self.objects.add(object),
            TuioEntity::Blob(blob) => self.blobs.add(blob),
        }
        self.current_session_id += 1;
    }

    fn remove(&mut self, entity: TuioEntity) {
        match entity {
            TuioEntity::Cursor(cursor) => self.cursors.remove(cursor.session_id()),
            TuioEntity::Object(object) => self.objects.remove(object.session_id()),
            TuioEntity::Blob(blob) => self.blobs.remove(blob.session_id()),
        }
    }

    fn current_session_id(&self) -> i32 {
        self.current_session_id
    }

    fn quit(&mut self) {
        self.cursors.clear();
        self.objects.clear();
        self.blobs.clear();
        self.update(&[]);
    }
}
