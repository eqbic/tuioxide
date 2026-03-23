#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Rotation {
    pub(crate) angle: f32,
    last_angle: f32,
    pub(crate) speed: f32,
    last_speed: f32,
    pub(crate) acceleration: f32,
}

impl Rotation {
    pub(crate) fn new(angle: f32, rotation_speed: f32, rotation_acceleration: f32) -> Self {
        Self {
            angle,
            last_angle: angle,
            speed: rotation_speed,
            last_speed: rotation_speed,
            acceleration: rotation_acceleration,
        }
    }

    pub(crate) fn update(&mut self, angle: f32, rotation_speed: f32, rotation_acceleration: f32) {
        self.last_angle = self.angle;
        self.last_speed = self.speed;
        self.angle = angle;
        if self.should_calculate_rotation(angle, rotation_speed) {
            self.calculate_rotation();
        } else {
            self.speed = rotation_speed;
            self.acceleration = rotation_acceleration
        }
    }

    fn calculate_rotation(&mut self) {
        self.speed = self.angle - self.last_angle;
        self.acceleration = self.last_speed - self.speed;
    }

    fn should_calculate_rotation(&self, angle: f32, rotation_speed: f32) -> bool {
        self.last_angle != angle && rotation_speed == 0.0
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // ── new ──────────────────────────────────────────────────────────────────

    #[test]
    fn new_stores_initial_angle() {
        let r = Rotation::new(1.5, 0.2, 0.1);
        assert_relative_eq!(r.angle, 1.5);
    }

    #[test]
    fn new_stores_initial_speed() {
        let r = Rotation::new(1.5, 0.2, 0.1);
        assert_relative_eq!(r.speed, 0.2);
    }

    #[test]
    fn new_stores_initial_acceleration() {
        let r = Rotation::new(1.5, 0.2, 0.1);
        assert_relative_eq!(r.acceleration, 0.1);
    }

    #[test]
    fn new_sets_last_angle_equal_to_angle() {
        // Verify indirectly: an update with the same angle and zero speed should NOT
        // trigger recalculation (because last_angle == angle). The supplied values
        // (speed=0, accel=0) are stored as-is.
        let mut r = Rotation::new(1.0, 0.5, 0.3);
        r.update(1.0, 0.0, 0.0);
        assert_relative_eq!(r.speed, 0.0);
        assert_relative_eq!(r.acceleration, 0.0);
    }

    // ── update with explicit (non-zero) rotation speed ────────────────────────

    #[test]
    fn update_explicit_speed_stores_provided_speed() {
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(1.0, 2.5, 1.0);
        assert_relative_eq!(r.speed, 2.5);
    }

    #[test]
    fn update_explicit_speed_stores_provided_acceleration() {
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(1.0, 2.5, 1.0);
        assert_relative_eq!(r.acceleration, 1.0);
    }

    #[test]
    fn update_explicit_speed_updates_angle() {
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(2.34, 1.0, 0.0);
        assert_relative_eq!(r.angle, 2.34);
    }

    #[test]
    fn update_explicit_speed_does_not_recalculate() {
        // Even when angle changes, a non-zero speed means we trust supplied values.
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(1.0, 99.0, 42.0);
        assert_relative_eq!(r.speed, 99.0);
        assert_relative_eq!(r.acceleration, 42.0);
    }

    #[test]
    fn update_explicit_speed_negative_speed_stored_verbatim() {
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(0.5, -3.0, -1.5);
        assert_relative_eq!(r.speed, -3.0);
        assert_relative_eq!(r.acceleration, -1.5);
    }

    // ── update with zero speed (triggers recalculation) ───────────────────────

    #[test]
    fn update_zero_speed_recalculates_speed_from_angle_delta() {
        // Initial angle = 0.0, update to angle 1.0 with zero speed.
        // Recalculated speed = new_angle - last_angle = 1.0 - 0.0 = 1.0.
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(1.0, 0.0, 0.0);
        assert_relative_eq!(r.speed, 1.0);
    }

    #[test]
    fn update_zero_speed_recalculates_negative_angle_delta() {
        // Moving from 1.0 → 0.3 with zero speed: delta = 0.3 - 1.0 = -0.7.
        let mut r = Rotation::new(1.0, 0.0, 0.0);
        r.update(0.3, 0.0, 0.0);
        assert_relative_eq!(r.speed, -0.7);
    }

    #[test]
    fn update_zero_speed_recalculates_acceleration_from_speed_delta() {
        // Initial angle=0, speed=0. Update: angle→1.0, zero speed.
        // Recalculated speed = 1.0 - 0.0 = 1.0. last_speed was 0.
        // acceleration = last_speed - new_speed = 0 - 1.0 = -1.0.
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(1.0, 0.0, 0.0);
        assert_relative_eq!(r.acceleration, -1.0);
    }

    #[test]
    fn update_zero_speed_angle_unchanged_no_recalc() {
        // If angle has not changed, should_calculate_rotation returns false even with
        // zero speed. The supplied values (speed=0, accel=0) are stored as-is.
        let mut r = Rotation::new(2.0, 1.5, 0.5);
        r.update(2.0, 0.0, 0.0);
        assert_relative_eq!(r.speed, 0.0);
        assert_relative_eq!(r.acceleration, 0.0);
    }

    #[test]
    fn update_zero_speed_second_recalc_uses_previous_speed() {
        // Step 1: angle 0→2 with zero speed → speed = 2, accel = 0 - 2 = -2.
        // Step 2: angle unchanged (same position) → no recalc.
        // Step 3: angle 2→5 with zero speed → delta = 3, speed = 3, accel = last_speed(2) - 3 = -1.
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(2.0, 0.0, 0.0); // recalc: speed=2, accel=-2
        r.update(2.0, 0.0, 0.0); // no recalc: speed stored as 0 (supplied)
        r.update(5.0, 0.0, 0.0); // recalc: delta=3, speed=3, accel = last_speed(0) - 3 = -3
        assert_relative_eq!(r.speed, 3.0);
        assert_relative_eq!(r.acceleration, -3.0);
    }

    #[test]
    fn update_zero_speed_large_angle_change() {
        let mut r = Rotation::new(0.0, 0.0, 0.0);
        r.update(2.134, 0.0, 0.0);
        assert_relative_eq!(r.speed, 2.134);
    }
}
