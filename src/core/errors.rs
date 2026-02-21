use rosc::{OscBundle, OscMessage, OscPacket};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TuioError {
    #[error("unknown address: {0:?}")]
    UnknownAddress(OscMessage),
    #[error("unknown message type: {0:?}")]
    UnknownMessageType(OscMessage),
    #[error("empty message at: {0:?}")]
    EmptyMessage(OscMessage),
    #[error("missing source name at: {0:?}")]
    MissingSource(OscMessage),
    #[error("missing one or more arguments at: {0:?}")]
    MissingArguments(OscMessage),
    #[error("wrong argument type at index {1} in: {0:?}")]
    WrongArgumentType(OscMessage, usize),
    #[error("missing one or more mandatory messages in: {0:?}")]
    IncompleteBundle(OscBundle),
    #[error("OSC packet is not a bundle: {0:?}")]
    NotABundle(OscPacket),
}
