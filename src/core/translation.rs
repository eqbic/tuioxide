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

    pub(crate) fn update_from_message(
        &mut self,
        position: Position,
        velocity: Velocity,
        acceleration: f32,
    ) {
        self.last_position = self.position;
        self.last_velocity = self.velocity;
        self.position = position;
        self.velocity = velocity;
        self.acceleration = acceleration;
    }

    pub(crate) fn update_from_position(&mut self, position: Position) {
        self.last_position = self.position;
        self.last_velocity = self.velocity;
        self.position = position;
        self.calculate_motion();
    }

    fn calculate_motion(&mut self) {
        self.velocity = self.position - self.last_position;
        self.acceleration = self.velocity.speed() - self.last_velocity.speed();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(x: f32, y: f32) -> Position {
        Position::new(x, y)
    }

    fn vel(x: f32, y: f32) -> Velocity {
        Velocity::new(x, y)
    }

    // ── new ──────────────────────────────────────────────────────────────────

    #[test]
    fn new_stores_initial_values() {
        let t = Translation::new(pos(0.2, 0.4), vel(1.0, 2.0), 0.5);
        assert_eq!(t.position, pos(0.2, 0.4));
        assert_eq!(t.velocity, vel(1.0, 2.0));
        assert_eq!(t.acceleration, 0.5);
    }

    // ── update_from_message ───────────────────────────────────────────────────

    #[test]
    fn update_from_message_stores_all_provided_values() {
        let mut t = Translation::new(pos(0.0, 0.0), vel(0.0, 0.0), 0.0);
        t.update_from_message(pos(0.1, 0.2), vel(0.5, 0.3), 0.8);
        assert_eq!(t.position, pos(0.1, 0.2));
        assert_eq!(t.velocity, vel(0.5, 0.3));
        assert_eq!(t.acceleration, 0.8);
    }

    #[test]
    fn update_from_message_stores_zero_velocity_verbatim() {
        // Zero velocity is a valid value, not a signal to recalculate.
        let mut t = Translation::new(pos(0.0, 0.0), vel(1.0, 1.0), 1.0);
        t.update_from_message(pos(3.0, 4.0), vel(0.0, 0.0), 0.0);
        assert_eq!(t.velocity, vel(0.0, 0.0));
        assert_eq!(t.acceleration, 0.0);
    }

    // ── update_from_position ──────────────────────────────────────────────────

    #[test]
    fn update_from_position_updates_position() {
        let mut t = Translation::new(pos(0.0, 0.0), vel(0.0, 0.0), 0.0);
        t.update_from_position(pos(1.0, 2.0));
        assert_eq!(t.position, pos(1.0, 2.0));
    }

    #[test]
    fn update_from_position_derives_velocity_from_delta() {
        // Moving from (0,0) to (0.3, 0.4): velocity should equal the position delta.
        let mut t = Translation::new(pos(0.0, 0.0), vel(0.0, 0.0), 0.0);
        t.update_from_position(pos(0.3, 0.4));
        assert!((t.velocity.x - 0.3).abs() < 1e-6, "vx={}", t.velocity.x);
        assert!((t.velocity.y - 0.4).abs() < 1e-6, "vy={}", t.velocity.y);
    }

    #[test]
    fn update_from_position_derives_speed_3_4_5() {
        // 3-4-5 right triangle: position delta (3,4) → speed = 5.
        let mut t = Translation::new(pos(0.0, 0.0), vel(0.0, 0.0), 0.0);
        t.update_from_position(pos(3.0, 4.0));
        assert!((t.velocity.speed() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn update_from_position_derives_acceleration_from_speed_delta() {
        // Moving from (0,0) to (3,4): new speed = 5, previous speed = 0.
        // acceleration = 5 - 0 = 5.
        let mut t = Translation::new(pos(0.0, 0.0), vel(0.0, 0.0), 0.0);
        t.update_from_position(pos(3.0, 4.0));
        assert!(
            (t.acceleration - 5.0).abs() < 1e-6,
            "accel={}",
            t.acceleration
        );
    }

    #[test]
    fn update_from_position_no_change_gives_zero_velocity_and_acceleration() {
        // Updating to the same position should yield zero velocity and zero acceleration.
        let mut t = Translation::new(pos(1.0, 1.0), vel(0.0, 0.0), 0.0);
        t.update_from_position(pos(1.0, 1.0));
        assert_eq!(t.velocity, vel(0.0, 0.0));
        assert_eq!(t.acceleration, 0.0);
    }

    #[test]
    fn update_from_position_acceleration_zero_when_speed_constant() {
        // Two consecutive moves with the same position delta (3,4) → speed stays 5.
        // acceleration = new_speed (5) - last_speed (5) = 0.
        let mut t = Translation::new(pos(0.0, 0.0), vel(0.0, 0.0), 0.0);
        t.update_from_position(pos(3.0, 4.0));
        t.update_from_position(pos(6.0, 8.0));
        assert!((t.velocity.x - 3.0).abs() < 1e-6);
        assert!((t.velocity.y - 4.0).abs() < 1e-6);
        assert!(t.acceleration.abs() < 1e-6, "accel={}", t.acceleration);
    }
}
