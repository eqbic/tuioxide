use crate::core::tuio_time::TuioTime;

/// Base container with attributes all tuio entities share.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Container {
    pub(crate) start_time: TuioTime,
    pub(crate) current_time: TuioTime,
    pub(crate) session_id: i32,
}

impl Container {
    pub(crate) fn new(start_time: &TuioTime, session_id: i32) -> Self {
        Self {
            start_time: *start_time,
            current_time: *start_time,
            session_id,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime) {
        self.current_time = *time;
    }
}
