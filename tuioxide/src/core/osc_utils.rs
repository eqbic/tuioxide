use rosc::{OscMessage, OscType};

use crate::core::{errors::TuioError, tuio_time::TuioTime};

// fn extract_arg<T, F>(message: &OscMessage, index: usize, extractor: F) -> Result<T, TuioError>
// where
//     F: FnOnce(&OscType) -> Option<T>,
// {
//     match message.args.get(index) {
//         Some(arg) => {
//             extractor(arg).ok_or_else(|| TuioError::WrongArgumentType(message.clone(), index))
//         }
//         None => Err(TuioError::MissingArguments(message.clone())),
//     }
// }

// pub fn extract_int(message: &OscMessage, index: usize) -> Result<i32, TuioError> {
//     extract_arg(message, index, |arg| match arg {
//         OscType::Int(val) => Some(*val),
//         _ => None,
//     })
// }

// pub fn extract_float(message: &OscMessage, index: usize) -> Result<f32, TuioError> {
//     extract_arg(message, index, |arg| match arg {
//         OscType::Float(val) => Some(*val),
//         _ => None,
//     })
// }

// pub fn extract_time(message: &OscMessage, index: usize) -> Result<TuioTime, TuioError> {
//     extract_arg(message, index, |arg| match arg {
//         OscType::Time(val) => Some(TuioTime::from(*val)),
//         _ => None,
//     })
// }

// pub fn extract_string(message: &OscMessage, index: usize) -> Result<String, TuioError> {
//     extract_arg(message, index, |arg| match arg {
//         OscType::String(val) => Some(val.to_owned()),
//         _ => None,
//     })
// }

#[derive(Debug, Clone, Copy)]
pub struct ArgCursor<'a> {
    message: &'a OscMessage,
    index: usize,
}

impl<'a> ArgCursor<'a> {
    pub fn new(message: &'a OscMessage, start_index: usize) -> Self {
        Self {
            message,
            index: start_index,
        }
    }

    pub fn next_int(&mut self) -> Result<i32, TuioError> {
        match self.message.args.get(self.index) {
            Some(OscType::Int(value)) => {
                self.index += 1;
                Ok(*value)
            }
            Some(_) => Err(TuioError::WrongArgumentType(
                self.message.clone(),
                self.index,
            )),
            None => Err(TuioError::MissingArguments(self.message.clone())),
        }
    }

    pub fn next_float(&mut self) -> Result<f32, TuioError> {
        match self.message.args.get(self.index) {
            Some(OscType::Float(value)) => {
                self.index += 1;
                Ok(*value)
            }
            Some(_) => Err(TuioError::WrongArgumentType(
                self.message.clone(),
                self.index,
            )),
            None => Err(TuioError::MissingArguments(self.message.clone())),
        }
    }

    pub fn next_string(&mut self) -> Result<String, TuioError> {
        match self.message.args.get(self.index) {
            Some(OscType::String(value)) => {
                self.index += 1;
                Ok(value.to_owned())
            }
            Some(_) => Err(TuioError::WrongArgumentType(
                self.message.clone(),
                self.index,
            )),
            None => Err(TuioError::MissingArguments(self.message.clone())),
        }
    }

    pub fn next_time(&mut self) -> Result<TuioTime, TuioError> {
        match self.message.args.get(self.index) {
            Some(OscType::Time(value)) => {
                self.index += 1;
                Ok(TuioTime::from(*value))
            }
            Some(_) => Err(TuioError::WrongArgumentType(
                self.message.clone(),
                self.index,
            )),
            None => Err(TuioError::MissingArguments(self.message.clone())),
        }
    }

    pub fn remaining(&self) -> usize {
        self.message.args.len() - self.index
    }
}
