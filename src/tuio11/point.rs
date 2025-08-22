use crate::common::{tuio_state::TuioState, tuio_time::TuioTime};

pub trait Point {
    fn start_time(&self) -> &TuioTime;
    fn current_time(&self) -> &TuioTime;
    fn set_current_time(&mut self, current_time: TuioTime);
    fn session_id(&self) -> u32;
    fn state(&self) -> &TuioState;
    fn set_state(&mut self, state: TuioState);
}
