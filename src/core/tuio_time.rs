use std::{ops, time::SystemTime};

use rosc::{OscTime, OscTimeError};

use crate::core::constants::{MICRO_PER_MILLI, MICRO_PER_SECOND, MILLI_PER_SECOND, UNIX_OFFSET};

/// A timestamp used throughout the TUIO protocol, storing time as a combination
/// of whole seconds and a microsecond sub-second component.
///
/// `TuioTime` is used to track when TUIO entities were created and last updated,
/// and can be converted to and from OSC time tags ([`OscTime`]).
///
/// # Examples
///
/// ```
/// use tuioxide::core::TuioTime;
///
/// let t = TuioTime::new(1, 500_000);
/// assert_eq!(t.seconds(), 1);
/// assert_eq!(t.micro_seconds(), 500_000);
/// assert_eq!(t.get_total_milliseconds(), 1500);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct TuioTime {
    seconds: i64,
    micro_seconds: i64,
}

impl TuioTime {
    /// Creates a new [`TuioTime`] with the given `seconds` and `micro_seconds` components.
    ///
    /// # Parameters
    ///
    /// - `seconds`: The whole-second part of the timestamp (relative to the Unix epoch).
    /// - `micro_seconds`: The sub-second part of the timestamp, in microseconds.
    pub fn new(seconds: i64, micro_seconds: i64) -> Self {
        Self {
            seconds,
            micro_seconds,
        }
    }

    /// Creates a [`TuioTime`] from the current system clock.
    ///
    /// Internally converts [`SystemTime::now()`] to an [`OscTime`] and then into a
    /// [`TuioTime`]. Returns an error if the system time cannot be represented as an
    /// OSC time tag.
    ///
    /// # Errors
    ///
    /// Returns an [`OscTimeError`] if the system time is before the OSC epoch or the
    /// conversion otherwise fails.
    pub fn from_system_time() -> Result<TuioTime, OscTimeError> {
        let osc_time = OscTime::try_from(SystemTime::now())?;
        Ok(TuioTime::from(osc_time))
    }

    /// Returns the whole-second component of this timestamp.
    pub fn seconds(&self) -> i64 {
        self.seconds
    }

    /// Returns the sub-second microsecond component of this timestamp.
    pub fn micro_seconds(&self) -> i64 {
        self.micro_seconds
    }

    /// Returns the total duration represented by this timestamp as milliseconds.
    ///
    /// Computed as `seconds * 1000 + micro_seconds / 1000`.
    pub fn get_total_milliseconds(&self) -> i64 {
        self.seconds * MILLI_PER_SECOND + self.micro_seconds / MICRO_PER_MILLI
    }
}

impl From<OscTime> for TuioTime {
    /// Converts an [`OscTime`] tag into a [`TuioTime`].
    ///
    /// The NTP epoch offset ([`UNIX_OFFSET`]) is subtracted from the seconds field so
    /// that the resulting `TuioTime` is relative to the Unix epoch (1970-01-01).
    fn from(time_tag: OscTime) -> Self {
        let seconds = time_tag.seconds as i64 - UNIX_OFFSET;
        let micro_seconds = ((time_tag.fractional as u64 * MICRO_PER_SECOND as u64) >> 32) as i64;
        Self {
            seconds,
            micro_seconds,
        }
    }
}

impl ops::Add<TuioTime> for TuioTime {
    type Output = TuioTime;

    /// Adds two [`TuioTime`] values together, carrying over any overflow in the
    /// microsecond component into the seconds component.
    fn add(self, time: TuioTime) -> Self::Output {
        let mut seconds = self.seconds + time.seconds;
        let mut micro_seconds = self.micro_seconds + time.micro_seconds;
        seconds += micro_seconds / MICRO_PER_SECOND;
        micro_seconds %= MILLI_PER_SECOND;
        Self {
            seconds,
            micro_seconds,
        }
    }
}

impl ops::Add<&TuioTime> for TuioTime {
    type Output = TuioTime;

    /// Adds a [`TuioTime`] reference to this value. Delegates to the owned [`Add`] impl.
    fn add(self, time: &TuioTime) -> Self::Output {
        self + *time
    }
}

impl ops::Add<i64> for TuioTime {
    type Output = TuioTime;

    /// Adds a raw microsecond offset to this [`TuioTime`], correctly handling
    /// both positive and negative offsets and carrying over into the seconds field.
    fn add(self, micro_secs: i64) -> Self::Output {
        let micro_sum = self.micro_seconds + micro_secs;
        let seconds = if micro_sum < 0 {
            self.seconds - 1
        } else {
            self.seconds + micro_sum / MICRO_PER_SECOND
        };

        let micro_seconds = (self.micro_seconds + micro_secs) % MICRO_PER_SECOND;
        Self {
            seconds,
            micro_seconds,
        }
    }
}

impl ops::Sub<TuioTime> for TuioTime {
    type Output = TuioTime;

    /// Subtracts one [`TuioTime`] from another, borrowing from the seconds component
    /// if the microsecond subtraction would underflow.
    fn sub(self, time: TuioTime) -> Self::Output {
        let mut seconds = self.seconds - time.seconds;
        let mut micro_seconds = self.micro_seconds - time.micro_seconds;
        if micro_seconds < 0 {
            micro_seconds += MICRO_PER_SECOND;
            seconds -= 1;
        }
        Self {
            seconds,
            micro_seconds,
        }
    }
}

impl ops::Sub<&TuioTime> for TuioTime {
    type Output = TuioTime;

    /// Subtracts a [`TuioTime`] reference from this value. Delegates to the owned [`Sub`] impl.
    fn sub(self, time: &TuioTime) -> Self::Output {
        self - *time
    }
}

impl ops::Sub<i64> for TuioTime {
    type Output = TuioTime;

    /// Subtracts a raw microsecond offset from this [`TuioTime`], borrowing from the
    /// seconds component when the microsecond result would be negative.
    fn sub(self, micro_sec: i64) -> Self::Output {
        let mut seconds = self.seconds - micro_sec / MICRO_PER_SECOND;
        let mut micro_seconds = self.micro_seconds - micro_sec % MICRO_PER_SECOND;
        if micro_seconds < 0 {
            micro_seconds += MICRO_PER_SECOND;
            seconds -= 1;
        }
        Self {
            micro_seconds,
            seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_tuio_times() {
        let a = TuioTime::new(2, 999999);
        let b = TuioTime::new(3, 15);
        assert_eq!(a + b, TuioTime::new(6, 14));
    }

    #[test]
    fn test_add_tuio_time_microsec() {
        let a = TuioTime::new(2, 999999);
        assert_eq!(a + 12, TuioTime::new(3, 11));
    }

    #[test]
    fn test_sub_two_tuio_times() {
        let a = TuioTime::new(1, 15);
        let b = TuioTime::new(0, 18);
        assert_eq!(a - b, TuioTime::new(0, 999997));
    }

    #[test]
    fn test_sub_tuio_times_microsec() {
        let a = TuioTime::new(1, 15);
        assert_eq!(a - 18, TuioTime::new(0, 999997));
    }
}
