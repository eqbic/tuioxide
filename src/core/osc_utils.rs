use rosc::{OscMessage, OscType};

use crate::core::{TuioError, TuioTime};

/// A cursor-style iterator over the arguments of an [`OscMessage`].
///
/// `ArgCursor` provides sequential, typed access to the OSC argument list of a
/// message. It keeps track of the current position and advances automatically
/// after each successful read, making it easy to decode structured TUIO messages
/// without manual index management.
///
/// # Example
///
/// ```ignore
/// let mut cursor = ArgCursor::new(&message, 1); // skip the message-type string at index 0
/// let session_id = cursor.next_int()?;
/// let x = cursor.next_float()?;
/// let y = cursor.next_float()?;
/// ```
#[derive(Debug, Clone, Copy)]
pub(crate) struct ArgCursor<'a> {
    message: &'a OscMessage,
    index: usize,
}

impl<'a> ArgCursor<'a> {
    /// Creates a new `ArgCursor` for the given `message`, starting at `start_index`.
    ///
    /// `start_index` is useful for skipping leading arguments such as the
    /// message-type string (e.g. `"set"`, `"alive"`) that TUIO prepends to most
    /// messages.
    pub(crate) fn new(message: &'a OscMessage, start_index: usize) -> Self {
        Self {
            message,
            index: start_index,
        }
    }

    /// Reads the next argument as an `i32` integer and advances the cursor.
    ///
    /// # Errors
    ///
    /// Returns [`TuioError::WrongArgumentType`] if the argument at the current
    /// position is not an [`OscType::Int`], or [`TuioError::MissingArguments`]
    /// if there are no more arguments.
    pub(crate) fn next_int(&mut self) -> Result<i32, TuioError> {
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

    /// Reads the next argument as an `f32` float and advances the cursor.
    ///
    /// # Errors
    ///
    /// Returns [`TuioError::WrongArgumentType`] if the argument at the current
    /// position is not an [`OscType::Float`], or [`TuioError::MissingArguments`]
    /// if there are no more arguments.
    pub(crate) fn next_float(&mut self) -> Result<f32, TuioError> {
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

    /// Reads the next argument as an owned [`String`] and advances the cursor.
    ///
    /// # Errors
    ///
    /// Returns [`TuioError::WrongArgumentType`] if the argument at the current
    /// position is not an [`OscType::String`], or [`TuioError::MissingArguments`]
    /// if there are no more arguments.
    pub(crate) fn next_string(&mut self) -> Result<String, TuioError> {
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

    /// Reads the next argument as a [`TuioTime`] (converted from an OSC time tag)
    /// and advances the cursor.
    ///
    /// # Errors
    ///
    /// Returns [`TuioError::WrongArgumentType`] if the argument at the current
    /// position is not an [`OscType::Time`], or [`TuioError::MissingArguments`]
    /// if there are no more arguments.
    pub(crate) fn next_time(&mut self) -> Result<TuioTime, TuioError> {
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

    /// Returns the number of arguments remaining from the current cursor position
    /// to the end of the message's argument list.
    ///
    /// This is useful for decoding optional trailing arguments that may or may not
    /// be present in a message.
    pub(crate) fn remaining(&self) -> usize {
        self.message.args.len() - self.index
    }
}
