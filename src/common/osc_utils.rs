use std::marker::Sized;
use std::ops::{Index, IndexMut};

use rosc::{OscMessage, OscType};

use crate::common::errors::TuioError;
use crate::common::tuio_time::TuioTime;

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

pub fn extract_time(message: &OscMessage, index: usize) -> Result<TuioTime, TuioError> {
    match message.args.get(index) {
        Some(arg) => match arg {
            OscType::Time(val) => Ok(TuioTime::from(*val)),
            _ => Err(TuioError::WrongArgumentType(message.clone(), index)),
        },
        None => Err(TuioError::MissingArguments(message.clone())),
    }
}

pub fn extract_string(message: &OscMessage, index: usize) -> Result<String, TuioError> {
    match message.args.get(index) {
        Some(arg) => match arg {
            OscType::String(val) => Ok((*val).clone()),
            _ => Err(TuioError::WrongArgumentType(message.clone(), index)),
        },
        None => Err(TuioError::MissingArguments(message.clone())),
    }
}

pub struct ProfileBuilder<T>
where
    T: Index<usize> + IndexMut<usize> + Sized,
{
    message: OscMessage,
    profile: T,
}

impl<T> ProfileBuilder<T>
where
    T: Index<usize> + IndexMut<usize> + Sized,
    T::Output: Sized,
{
    pub fn new(message: OscMessage, profile: T) -> Self {
        Self { message, profile }
    }

    // pub fn with_int(&mut self, index: usize) -> Result<&mut Self, TuioError> {
    //     let value = extract_int(&self.message, index)?;
    //     self.profile[index] = value;
    //     Ok(self)
    // }
}
