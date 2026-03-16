use crate::core::math::{Position, Velocity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Translation {
    pub(crate) position: Position,
    last_position: Position,
    pub(crate) velocity: Velocity,
    last_velocity: Velocity,
    pub(crate) acceleration: f32,
}

impl Translation {
    pub(crate) fn new(position: Position, velocity: Velocity, acceleration: f32) -> Self {
        Self {
            position,
            last_position: position,
            velocity,
            last_velocity: velocity,
            acceleration,
        }
    }

    pub(crate) fn update(&mut self, position: Position, velocity: Velocity, acceleration: f32) {
        self.last_position = self.position;
        self.last_velocity = self.velocity;
        self.position = position;
        if self.should_calculate_motion(position, velocity) {
            self.calculate_motion();
        } else {
            self.velocity = velocity;
            self.acceleration = acceleration;
        }
    }

    fn calculate_motion(&mut self) {
        self.velocity = self.position - self.last_position;
        self.acceleration = self.velocity.speed() - self.last_velocity.speed();
    }

    fn should_calculate_motion(&self, position: Position, velocity: Velocity) -> bool {
        self.last_position != position && velocity.speed() == 0.0
    }
}
