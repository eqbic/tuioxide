use rosc::{OscMessage, OscType};

use crate::common::errors::TuioError;
use crate::common::tuio_time::TuioTime;

fn extract_arg<T, F>(message: &OscMessage, index: usize, extractor: F) -> Result<T, TuioError>
where
    F: FnOnce(&OscType) -> Option<T>,
{
    match message.args.get(index) {
        Some(arg) => {
            extractor(arg).ok_or_else(|| TuioError::WrongArgumentType(message.clone(), index))
        }
        None => Err(TuioError::MissingArguments(message.clone())),
    }
}

pub fn extract_int(message: &OscMessage, index: usize) -> Result<i32, TuioError> {
    extract_arg(message, index, |arg| match arg {
        OscType::Int(val) => Some(*val),
        _ => None,
    })
}

pub fn extract_float(message: &OscMessage, index: usize) -> Result<f32, TuioError> {
    extract_arg(message, index, |arg| match arg {
        OscType::Float(val) => Some(*val),
        _ => None,
    })
}

pub fn extract_time(message: &OscMessage, index: usize) -> Result<TuioTime, TuioError> {
    extract_arg(message, index, |arg| match arg {
        OscType::Time(val) => Some(TuioTime::from(*val)),
        _ => None,
    })
}

pub fn extract_string(message: &OscMessage, index: usize) -> Result<String, TuioError> {
    extract_arg(message, index, |arg| match arg {
        OscType::String(val) => Some(val.to_owned()),
        _ => None,
    })
}
