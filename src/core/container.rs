use crate::core::{
    math::{Position, Velocity},
    profile::Profile,
    tuio_time::TuioTime,
};

/// Base container with attributes all tuio entities share.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Container {
    pub(crate) start_time: TuioTime,
    pub(crate) current_time: TuioTime,
    pub(crate) session_id: i32,
    pub(crate) position: Position,
    last_position: Position,
    pub(crate) velocity: Velocity,
    pub(crate) acceleration: f32,
}

impl Container {
    pub(crate) fn new(start_time: &TuioTime, session_id: i32, position: Position) -> Self {
        Self {
            start_time: *start_time,
            current_time: *start_time,
            session_id,
            position,
            last_position: Position::default(),
            velocity: Velocity::default(),
            acceleration: 0.0,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, profile: &impl Profile) {
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

    fn calculate_motion(&mut self, position: Position) {
        self.velocity = position - self.last_position;
        self.acceleration = self.velocity.speed()
    }

    fn should_calculate_motion(&self, position: Position, velocity: Velocity) -> bool {
        self.last_position != position && velocity.speed() == 0.0
    }
}
