use crate::core::common::tuio_time::TuioTime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Container {
    start_time: TuioTime,
    current_time: TuioTime,
}

impl Container {
    pub fn new(start_time: &TuioTime) -> Self {
        Self {
            start_time: *start_time,
            current_time: *start_time,
        }
    }

    pub fn update(&mut self, time: &TuioTime) {
        self.current_time = *time
    }

    pub fn start_time(&self) -> &TuioTime {
        &self.start_time
    }

    pub fn current_time(&self) -> &TuioTime {
        &self.current_time
    }
}
