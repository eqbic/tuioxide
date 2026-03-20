use crate::tuio11::{Blob, Cursor, Object};

#[derive(Debug, Clone, Copy)]
pub enum TuioEntity {
    Cursor(Cursor),
    Object(Object),
    Blob(Blob),
}

impl TuioEntity {
    pub fn session_id(&self) -> i32 {
        match self {
            TuioEntity::Cursor(cursors) => cursors.session_id(),
            TuioEntity::Object(object) => object.session_id(),
            TuioEntity::Blob(blob) => blob.session_id(),
        }
    }
}
