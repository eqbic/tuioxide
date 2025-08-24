use rosc::{OscMessage, OscType};

use crate::common::errors::TuioError;

pub fn extract_int(message: &OscMessage, index: usize) -> Result<i32, TuioError> {
    match message.args.get(index) {
        Some(arg) => match arg {
            OscType::Int(val) => Ok(*val),
            _ => Err(TuioError::WrongArgumentType(message.clone(), index)),
        },
        None => Err(TuioError::MissingArguments(message.clone())),
    }
}

pub fn extract_float(message: &OscMessage, index: usize) -> Result<f32, TuioError> {
    match message.args.get(index) {
        Some(arg) => match arg {
            OscType::Float(val) => Ok(*val),
            _ => Err(TuioError::WrongArgumentType(message.clone(), index)),
        },
        None => Err(TuioError::MissingArguments(message.clone())),
    }
}
