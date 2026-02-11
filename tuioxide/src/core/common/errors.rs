use std::{error, fmt, io};

use rosc::{OscBundle, OscError, OscMessage, OscPacket};
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

// impl fmt::Display for TuioError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TuioError::UnknownAddress(msg) => write!(f, "unknown address: {:?}", msg.addr),
//             TuioError::UnknownMessageType(msg) => write!(f, "unknown message type: {:?}", msg),
//             TuioError::EmptyMessage(msg) => write!(f, "empty message at: {:?}", msg),
//             TuioError::MissingSource(msg) => write!(f, "missing source name at: {:?}", msg),
//             TuioError::MissingArguments(msg) => {
//                 write!(f, "missing one or more arguments at: {:?}", msg)
//             }
//             TuioError::WrongArgumentType(msg, index) => {
//                 write!(f, "wrong argument type at index {} in: {:?}", index, msg)
//             }
//             TuioError::IncompleteBundle(bundle) => {
//                 write!(f, "missing one or more mandatory messages in: {:?}", bundle)
//             }
//             TuioError::NotABundle(packet) => write!(f, "OSC packet is not a bundle: {:?}", packet),
//         }
//     }
// }

#[derive(Debug)]
pub enum OscReceiverError {
    Connect(io::Error),
    AlreadyConnected(),
    Receive(io::Error),
    Decode(OscError),
}

impl fmt::Display for OscReceiverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OscReceiverError::AlreadyConnected() => write!(f, "OscReceiver is already connected"),
            OscReceiverError::Connect(msg) => write!(f, "error connecting OscReceiver: {}", msg),
            OscReceiverError::Receive(msg) => write!(f, "error receiving OSC packet: {}", msg),
            OscReceiverError::Decode(msg) => write!(f, "error decoding OSC packet: {}", msg),
        }
    }
}

impl error::Error for OscReceiverError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            OscReceiverError::AlreadyConnected() => None,
            OscReceiverError::Connect(err) => Some(err),
            OscReceiverError::Receive(err) => Some(err),
            OscReceiverError::Decode(err) => Some(err),
        }
    }
}
