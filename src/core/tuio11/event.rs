use crate::core::tuio11::{blob::Blob, cursor::Cursor, object::Object};

#[derive(Debug)]
pub enum CursorEvent {
    Add(Cursor),
    Update(Cursor),
    Remove(Cursor),
}

#[derive(Debug)]
pub enum ObjectEvent {
    Add(Object),
    Update(Object),
    Remove(Object),
}

#[derive(Debug)]
pub enum BlobEvent {
    Add(Blob),
    Update(Blob),
    Remove(Blob),
}
