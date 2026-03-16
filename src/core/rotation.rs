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
