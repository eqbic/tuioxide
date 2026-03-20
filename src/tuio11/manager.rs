use rosc::OscBundle;

use crate::tuio11::{Blob, Cursor, Object, entity::TuioEntity, repository::TuioRepository};

struct Manager {
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

    pub fn update(&mut self) -> &Vec<OscBundle> {
        self.frame_id += 1;
        self.frame_bundles[0] = self.cursors.update(self.frame_id);
        self.frame_bundles[1] = self.objects.update(self.frame_id);
        self.frame_bundles[2] = self.blobs.update(self.frame_id);
        &self.frame_bundles
    }

    pub fn add(&mut self, entity: TuioEntity) {
        match entity {
            TuioEntity::Cursor(cursor) => self.cursors.add(cursor),
            TuioEntity::Object(object) => self.objects.add(object),
            TuioEntity::Blob(blob) => self.blobs.add(blob),
        }
    }

    pub fn remove(&mut self, entity: TuioEntity) {
        match entity {
            TuioEntity::Cursor(cursor) => self.cursors.remove(cursor.session_id()),
            TuioEntity::Object(object) => self.objects.remove(object.session_id()),
            TuioEntity::Blob(blob) => self.blobs.remove(blob.session_id()),
        }
    }
}
