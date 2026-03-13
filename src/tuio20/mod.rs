mod bounds;
pub(crate) mod bundle;
mod events;
pub(crate) mod osc_decoder;
mod pointer;
pub(crate) mod processor;
mod symbol;
mod token;

pub use bounds::Bounds;
pub use bundle::Frame;
pub use events::*;
pub use pointer::Pointer;
pub(crate) use processor::Processor;
pub use symbol::Symbol;
pub use token::Token;

/// A TUIO 2.0 client. See [`core::Client`] for full documentation.
pub type Client = crate::core::Client<crate::tuio20::Processor>;
