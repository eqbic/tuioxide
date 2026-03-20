use std::io;

use rosc::OscPacket;

use crate::{
    core::{Position, Size, Velocity, osc_sender::OscSender},
    tuio11::{Blob, Cursor, Object, repository::TuioRepository},
};

pub struct Server {
    senders: Vec<Box<dyn OscSender>>,
    cursors: TuioRepository<Cursor>,
    objects: TuioRepository<Object>,
    blobs: TuioRepository<Blob>,
    next_session_id: i32,
    frame_id: i32,
}

impl Server {
    pub fn new(source_name: &str) -> Self {
        Self {
            senders: Vec::new(),
            cursors: TuioRepository::new(source_name),
            objects: TuioRepository::new(source_name),
            blobs: TuioRepository::new(source_name),
            next_session_id: 0,
            frame_id: 0,
        }
    }

    pub fn next_session_id(&self) -> i32 {
        self.next_session_id
    }

    pub fn add_sender(&mut self, sender: impl OscSender + 'static) {
        self.senders.push(Box::new(sender));
    }

    pub fn send_frame(&mut self) -> Result<(), std::io::Error> {
        self.frame_id += 1;
        let bundles = [
            OscPacket::Bundle(self.cursors.bundle(self.frame_id)),
            OscPacket::Bundle(self.objects.bundle(self.frame_id)),
            OscPacket::Bundle(self.blobs.bundle(self.frame_id)),
        ];
        for sender in &self.senders {
            for bundle in &bundles {
                sender.send(bundle)?;
            }
        }
        Ok(())
    }

    pub fn quit(&mut self) -> Result<(), io::Error> {
        self.cursors.clear();
        self.objects.clear();
        self.blobs.clear();
        self.send_frame()?;
        Ok(())
    }

    pub fn add_cursor(&mut self, session_id: i32, position: Position) -> Cursor {
        let cursor = Cursor::new(session_id, position, Velocity::default(), 0.0);
        self.cursors.add(cursor);
        self.next_session_id += 1;
        cursor
    }

    pub fn update_cursor(&mut self, cursor: Cursor) {
        self.cursors.update(cursor)
    }

    pub fn remove_cursor(&mut self, cursor: Cursor) {
        self.cursors.remove(cursor.session_id());
    }

    pub fn add_object(
        &mut self,
        session_id: i32,
        class_id: i32,
        position: Position,
        angle: f32,
    ) -> Object {
        let object = Object::new(
            session_id,
            class_id,
            position,
            Velocity::default(),
            0.0,
            angle,
            0.0,
            0.0,
        );
        self.objects.add(object);
        self.next_session_id += 1;
        object
    }

    pub fn update_object(&mut self, object: Object) {
        self.objects.update(object);
    }

    pub fn remove_object(&mut self, object: Object) {
        self.objects.remove(object.session_id());
    }

    pub fn add_blob(
        &mut self,
        session_id: i32,
        position: Position,
        angle: f32,
        size: Size,
        area: f32,
    ) -> Blob {
        let blob = Blob::new(
            session_id,
            position,
            Velocity::default(),
            0.0,
            angle,
            0.0,
            0.0,
            size,
            area,
        );
        self.blobs.add(blob);
        self.next_session_id += 1;
        blob
    }

    pub fn update_blob(&mut self, blob: Blob) {
        self.blobs.update(blob);
    }

    pub fn remove_blob(&mut self, blob: Blob) {
        self.blobs.remove(blob.session_id());
    }
}
