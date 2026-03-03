use crate::core::tuio11::{blob::Blob, cursor::Cursor, object::Object};

/// An event emitted when a TUIO 1.1 cursor (fingertip / touch point) changes state.
///
/// Cursors are tracked via the `/tuio/2Dcur` OSC address and represent individual
/// touch contacts on a surface.
#[derive(Debug)]
pub enum CursorEvent {
    /// A new cursor has appeared and been added to the active session.
    Add(Cursor),
    /// An existing cursor has moved or changed its motion properties.
    Update(Cursor),
    /// A cursor is no longer active and has been removed from the session.
    Remove(Cursor),
}

/// An event emitted when a TUIO 1.1 object (tagged tangible) changes state.
///
/// Objects are tracked via the `/tuio/2Dobj` OSC address and represent physical
/// objects with a known class ID placed on a surface.
#[derive(Debug)]
pub enum ObjectEvent {
    /// A new object has appeared and been added to the active session.
    Add(Object),
    /// An existing object has moved, rotated, or changed its motion properties.
    Update(Object),
    /// An object is no longer active and has been removed from the session.
    Remove(Object),
}

/// An event emitted when a TUIO 1.1 blob (unidentified contact region) changes state.
///
/// Blobs are tracked via the `/tuio/2Dblb` OSC address and represent amorphous
/// contact areas on a surface that carry size and area information in addition to
/// position and rotation.
#[derive(Debug)]
pub enum BlobEvent {
    /// A new blob has appeared and been added to the active session.
    Add(Blob),
    /// An existing blob has moved, resized, or changed its motion properties.
    Update(Blob),
    /// A blob is no longer active and has been removed from the session.
    Remove(Blob),
}
