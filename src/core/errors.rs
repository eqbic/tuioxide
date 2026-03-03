use rosc::{OscBundle, OscMessage, OscPacket};
use thiserror::Error;

/// Represents errors that can occur during TUIO processing and decoding.
#[derive(Debug, Error)]
pub enum TuioError {
    /// The OSC address is not recognized as a valid TUIO address.
    #[error("unknown address: {0:?}")]
    UnknownAddress(OscMessage),
    /// The TUIO message type (e.g., "set", "alive", "fseq") is unknown.
    #[error("unknown message type: {0:?}")]
    UnknownMessageType(OscMessage),
    /// The TUIO message contains no arguments.
    #[error("empty message at: {0:?}")]
    EmptyMessage(OscMessage),
    /// The source message is missing the source name argument.
    #[error("missing source name at: {0:?}")]
    MissingSource(OscMessage),
    /// The message is missing one or more required arguments.
    #[error("missing one or more arguments at: {0:?}")]
    MissingArguments(OscMessage),
    /// An argument has an unexpected type at the specified index.
    #[error("wrong argument type at index {1} in: {0:?}")]
    WrongArgumentType(OscMessage, usize),
    /// The TUIO bundle is missing mandatory messages (e.g., "alive" or "fseq").
    #[error("missing one or more mandatory messages in: {0:?}")]
    IncompleteBundle(OscBundle),
    /// The received OSC packet is not an OSC bundle.
    #[error("OSC packet is not a bundle: {0:?}")]
    NotABundle(OscPacket),
}
