mod blob;
pub(crate) mod bundle;
mod cursor;
mod events;
mod manager;
mod object;
pub(crate) mod osc_decoder_encoder;
pub(crate) mod processor;
mod repository;

pub use blob::Blob;
pub use cursor::Cursor;
pub use events::*;
pub use object::Object;
pub(crate) use processor::Processor;
mod entity;

/// A TUIO 1.1 client. See [`core::Client`] for full documentation.
pub type Client = crate::core::Client<crate::tuio11::Processor>;
