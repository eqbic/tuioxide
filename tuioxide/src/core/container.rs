use crate::core::{
    math::{Position, Velocity},
    tuio_state::TuioState,
    tuio_time::TuioTime,
    tuio11::profile::Profile,
};

/// Base container with attributes all tuio entities share.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Container {
    start_time: TuioTime,
    current_time: TuioTime,
    state: TuioState,
    session_id: i32,
    position: Position,
    last_position: Position,
    velocity: Velocity,
    acceleration: f32,
}

impl Container {
    pub fn new(start_time: &TuioTime, session_id: i32, position: Position) -> Self {
        Self {
            start_time: *start_time,
            current_time: *start_time,
            state: TuioState::Added,
            session_id,
            position,
            last_position: Position::default(),
            velocity: Velocity::default(),
            acceleration: 0.0,
        }
    }

    pub fn update(&mut self, time: &TuioTime, profile: &impl Profile) {
        self.current_time = *time;
        self.last_position = self.position;
        self.position = profile.position();
        if self.should_calculate_motion(self.position, profile.velocity()) {
            self.calculate_motion(self.position);
        } else {
            self.velocity = profile.velocity();
            self.acceleration = profile.acceleration();
        }
    }

    pub fn start_time(&self) -> TuioTime {
        self.start_time
    }

    pub fn current_time(&self) -> TuioTime {
        self.current_time
    }

    pub fn state(&self) -> TuioState {
        self.state
    }

    pub fn session_id(&self) -> i32 {
        self.session_id
    }

    pub fn position(&self) -> Position {
        self.position
    }

    fn calculate_motion(&mut self, position: Position) {
        self.velocity = position - self.last_position;
        self.acceleration = self.velocity.speed()
    }

    fn should_calculate_motion(&self, position: Position, velocity: Velocity) -> bool {
        self.position != position && velocity.speed() == 0.0
    }
}
