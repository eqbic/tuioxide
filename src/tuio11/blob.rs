use euclid::default::{Point2D, Vector2D};

use crate::{
    common::{tuio_state::TuioState, tuio_time::TuioTime},
    tuio11::{point::Point, rotation::Rotation, translation::Translation},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Blob {
    state: TuioState,
    start_time: TuioTime,
    current_time: TuioTime,
    session_id: u32,
    position: Point2D<f32>,
    velocity: Vector2D<f32>,
    speed: f32,
    acceleration: f32,
    angle: f32,
    blob_id: u32,
    size: Vector2D<f32>,
    area: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
}

impl Point for Blob {
    fn start_time(&self) -> &TuioTime {
        &self.start_time
    }

    fn current_time(&self) -> &TuioTime {
        &self.current_time
    }

    fn set_current_time(&mut self, current_time: TuioTime) {
        self.current_time = current_time
    }

    fn session_id(&self) -> u32 {
        self.session_id
    }

    fn state(&self) -> &TuioState {
        &self.state
    }

    fn set_state(&mut self, state: TuioState) {
        self.state = state
    }
}

impl Translation for Blob {
    fn position(&self) -> &Point2D<f32> {
        &self.position
    }

    fn velocity(&self) -> &Vector2D<f32> {
        &self.velocity
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn set_position(&mut self, position: Point2D<f32>) {
        self.position = position
    }

    fn set_velocity(&mut self, velocity: Vector2D<f32>) {
        self.velocity = velocity
    }

    fn set_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration
    }

    fn set_speed(&mut self, speed: f32) {
        self.speed = speed
    }
}

impl Rotation for Blob {
    fn angle(&self) -> f32 {
        self.angle
    }

    fn set_angle(&mut self, angle: f32) {
        self.angle = angle
    }

    fn rotation_speed(&self) -> f32 {
        self.rotation_speed
    }

    fn set_rotation_speed(&mut self, rotation_speed: f32) {
        self.rotation_speed = rotation_speed
    }

    fn rotation_acceleration(&self) -> f32 {
        self.rotation_acceleration
    }

    fn set_rotation_acceleration(&mut self, rotation_acceleration: f32) {
        self.rotation_acceleration = rotation_acceleration
    }
}

impl Blob {
    pub fn new(
        start_time: TuioTime,
        session_id: u32,
        blob_id: u32,
        position: Point2D<f32>,
        angle: f32,
        size: Vector2D<f32>,
        area: f32,
        velocity: Vector2D<f32>,
        rotation_speed: f32,
        acceleration: f32,
        rotation_acceleration: f32,
    ) -> Self {
        Self {
            state: TuioState::Added,
            current_time: start_time.clone(),
            start_time,
            session_id,
            blob_id,
            position,
            velocity,
            speed: velocity.length(),
            acceleration,
            angle,
            size,
            area,
            rotation_acceleration,
            rotation_speed,
        }
    }

    pub fn update(
        &mut self,
        current_time: TuioTime,
        position: Point2D<f32>,
        angle: f32,
        size: Vector2D<f32>,
        area: f32,
        velocity: Vector2D<f32>,
        rotation_speed: f32,
        acceleration: f32,
        rotation_acceleration: f32,
    ) {
        self.update_translation(current_time, position, velocity, acceleration);
        self.update_rotation(current_time, angle, rotation_speed, rotation_acceleration);
        self.size = size;
        self.area = area;
    }
}
