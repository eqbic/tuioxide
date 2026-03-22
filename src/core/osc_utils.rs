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

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;
    use rosc::{OscMessage, OscTime, OscType};

    use crate::core::TuioError;

    use super::ArgCursor;

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn make_msg(args: Vec<OscType>) -> OscMessage {
        OscMessage {
            addr: "/test".to_string(),
            args,
        }
    }

    // ── next_int ─────────────────────────────────────────────────────────────

    #[test]
    fn next_int_reads_integer() {
        let msg = make_msg(vec![OscType::Int(42)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.next_int().unwrap(), 42);
    }

    #[test]
    fn next_int_advances_cursor() {
        let msg = make_msg(vec![OscType::Int(1), OscType::Int(2)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.next_int().unwrap(), 1);
        assert_eq!(cursor.next_int().unwrap(), 2);
    }

    #[test]
    fn next_int_wrong_type_returns_error() {
        let msg = make_msg(vec![OscType::Float(1.0)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_int().unwrap_err();
        assert!(
            matches!(err, TuioError::WrongArgumentType(_, 0)),
            "expected WrongArgumentType, got {err:?}"
        );
    }

    #[test]
    fn next_int_missing_returns_error() {
        let msg = make_msg(vec![]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_int().unwrap_err();
        assert!(
            matches!(err, TuioError::MissingArguments(_)),
            "expected MissingArguments, got {err:?}"
        );
    }

    #[test]
    fn next_int_wrong_type_error_reports_correct_index() {
        let msg = make_msg(vec![OscType::Int(1), OscType::Float(2.0)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let _ = cursor.next_int().unwrap(); // consume index 0
        let err = cursor.next_int().unwrap_err();
        assert!(
            matches!(err, TuioError::WrongArgumentType(_, 1)),
            "expected index 1, got {err:?}"
        );
    }

    // ── next_float ───────────────────────────────────────────────────────────

    #[test]
    fn next_float_reads_float() {
        let msg = make_msg(vec![OscType::Float(0.345)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_relative_eq!(cursor.next_float().unwrap(), 0.345);
    }

    #[test]
    fn next_float_advances_cursor() {
        let msg = make_msg(vec![OscType::Float(1.0), OscType::Float(2.0)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_relative_eq!(cursor.next_float().unwrap(), 1.0);
        assert_relative_eq!(cursor.next_float().unwrap(), 2.0);
    }

    #[test]
    fn next_float_wrong_type_returns_error() {
        let msg = make_msg(vec![OscType::Int(1)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_float().unwrap_err();
        assert!(
            matches!(err, TuioError::WrongArgumentType(_, 0)),
            "expected WrongArgumentType, got {err:?}"
        );
    }

    #[test]
    fn next_float_missing_returns_error() {
        let msg = make_msg(vec![]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_float().unwrap_err();
        assert!(
            matches!(err, TuioError::MissingArguments(_)),
            "expected MissingArguments, got {err:?}"
        );
    }

    // ── next_string ──────────────────────────────────────────────────────────

    #[test]
    fn next_string_reads_string() {
        let msg = make_msg(vec![OscType::String("hello".to_string())]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.next_string().unwrap(), "hello");
    }

    #[test]
    fn next_string_advances_cursor() {
        let msg = make_msg(vec![
            OscType::String("first".to_string()),
            OscType::String("second".to_string()),
        ]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.next_string().unwrap(), "first");
        assert_eq!(cursor.next_string().unwrap(), "second");
    }

    #[test]
    fn next_string_wrong_type_returns_error() {
        let msg = make_msg(vec![OscType::Int(99)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_string().unwrap_err();
        assert!(
            matches!(err, TuioError::WrongArgumentType(_, 0)),
            "expected WrongArgumentType, got {err:?}"
        );
    }

    #[test]
    fn next_string_missing_returns_error() {
        let msg = make_msg(vec![]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_string().unwrap_err();
        assert!(
            matches!(err, TuioError::MissingArguments(_)),
            "expected MissingArguments, got {err:?}"
        );
    }

    // ── next_time ────────────────────────────────────────────────────────────

    #[test]
    fn next_time_reads_time() {
        // Use a fixed OscTime and verify it converts without error.
        let osc_time = OscTime {
            seconds: 3_000_000_000,
            fractional: 0,
        };
        let msg = make_msg(vec![OscType::Time(osc_time)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        // Should succeed and return a TuioTime (exact value tested elsewhere).
        let _time = cursor.next_time().unwrap();
    }

    #[test]
    fn next_time_wrong_type_returns_error() {
        let msg = make_msg(vec![OscType::Float(1.0)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_time().unwrap_err();
        assert!(
            matches!(err, TuioError::WrongArgumentType(_, 0)),
            "expected WrongArgumentType, got {err:?}"
        );
    }

    #[test]
    fn next_time_missing_returns_error() {
        let msg = make_msg(vec![]);
        let mut cursor = ArgCursor::new(&msg, 0);
        let err = cursor.next_time().unwrap_err();
        assert!(
            matches!(err, TuioError::MissingArguments(_)),
            "expected MissingArguments, got {err:?}"
        );
    }

    // ── remaining ────────────────────────────────────────────────────────────

    #[test]
    fn remaining_counts_args_from_start() {
        let msg = make_msg(vec![OscType::Int(1), OscType::Int(2), OscType::Int(3)]);
        let cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.remaining(), 3);
    }

    #[test]
    fn remaining_counts_from_start_index() {
        let msg = make_msg(vec![OscType::Int(1), OscType::Int(2), OscType::Int(3)]);
        // Skip the first arg (e.g. a message-type string).
        let cursor = ArgCursor::new(&msg, 1);
        assert_eq!(cursor.remaining(), 2);
    }

    #[test]
    fn remaining_decreases_after_reads() {
        let msg = make_msg(vec![OscType::Int(10), OscType::Int(20), OscType::Int(30)]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.remaining(), 3);
        let _ = cursor.next_int().unwrap();
        assert_eq!(cursor.remaining(), 2);
        let _ = cursor.next_int().unwrap();
        assert_eq!(cursor.remaining(), 1);
        let _ = cursor.next_int().unwrap();
        assert_eq!(cursor.remaining(), 0);
    }

    #[test]
    fn remaining_zero_on_empty_message() {
        let msg = make_msg(vec![]);
        let cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.remaining(), 0);
    }

    // ── start_index skipping ─────────────────────────────────────────────────

    #[test]
    fn start_index_skips_leading_args() {
        // Mimics TUIO usage: index 0 is the message-type string "set"; we start at 1.
        let msg = make_msg(vec![
            OscType::String("set".to_string()),
            OscType::Int(5),
            OscType::Float(0.5),
        ]);
        let mut cursor = ArgCursor::new(&msg, 1);
        assert_eq!(cursor.next_int().unwrap(), 5);
        assert_relative_eq!(cursor.next_float().unwrap(), 0.5);
        assert_eq!(cursor.remaining(), 0);
    }

    // ── mixed sequential reads ────────────────────────────────────────────────

    #[test]
    fn sequential_mixed_reads() {
        let osc_time = OscTime {
            seconds: 3_000_000_000,
            fractional: 0,
        };
        let msg = make_msg(vec![
            OscType::Int(42),
            OscType::Float(1.5),
            OscType::String("hello".to_string()),
            OscType::Time(osc_time),
        ]);
        let mut cursor = ArgCursor::new(&msg, 0);
        assert_eq!(cursor.next_int().unwrap(), 42);
        assert_relative_eq!(cursor.next_float().unwrap(), 1.5);
        assert_eq!(cursor.next_string().unwrap(), "hello");
        let _t = cursor.next_time().unwrap();
        assert_eq!(cursor.remaining(), 0);
    }
}
