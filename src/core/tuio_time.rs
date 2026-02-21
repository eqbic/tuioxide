use std::{ops, time::SystemTime};

use rosc::{OscTime, OscTimeError};

use crate::core::constants::{MICRO_PER_MILLI, MICRO_PER_SECOND, MILLI_PER_SECOND, UNIX_OFFSET};

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct TuioTime {
    seconds: i64,
    micro_seconds: i64,
}

impl TuioTime {
    pub fn new(seconds: i64, micro_seconds: i64) -> Self {
        Self {
            seconds,
            micro_seconds,
        }
    }

    pub fn from_system_time() -> Result<TuioTime, OscTimeError> {
        let osc_time = OscTime::try_from(SystemTime::now())?;
        Ok(TuioTime::from(osc_time))
    }

    pub fn seconds(&self) -> i64 {
        self.seconds
    }

    pub fn micro_seconds(&self) -> i64 {
        self.micro_seconds
    }

    pub fn get_total_milliseconds(&self) -> i64 {
        self.seconds * MILLI_PER_SECOND + self.micro_seconds / MICRO_PER_MILLI
    }
}

impl From<OscTime> for TuioTime {
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

    fn add(self, time: &TuioTime) -> Self::Output {
        self + *time
    }
}

impl ops::Add<i64> for TuioTime {
    type Output = TuioTime;

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

    fn sub(self, time: &TuioTime) -> Self::Output {
        self - *time
    }
}

impl ops::Sub<i64> for TuioTime {
    type Output = TuioTime;

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
