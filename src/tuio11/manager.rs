use rosc::OscBundle;

use crate::tuio11::{Blob, Cursor, Object, repository::TuioRepository};

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

    pub fn add_cursor(&mut self, cursor: Cursor) {
        self.cursors.add(cursor);
    }

    pub fn remove_cursor(&mut self, session_id: i32) {
        self.cursors.remove(session_id);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.add(object);
    }

    pub fn remove_object(&mut self, session_id: i32) {
        self.objects.remove(session_id);
    }

    pub fn add_blob(&mut self, blob: Blob) {
        self.blobs.add(blob);
    }

    pub fn remove_blob(&mut self, session_id: i32) {
        self.blobs.remove(session_id);
    }
}
