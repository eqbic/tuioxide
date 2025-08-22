use euclid::default::{Point2D, Vector2D};

use crate::{
    common::{tuio_state::TuioState, tuio_time::TuioTime},
    tuio11::{point::Point, translation::Translation},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Cursor {
    state: TuioState,
    start_time: TuioTime,
    current_time: TuioTime,
    session_id: u32,
    cursor_id: u32,
    position: Point2D<f32>,
    velocity: Vector2D<f32>,
    speed: f32,
    acceleration: f32,
}

impl Point for Cursor {
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

impl Translation for Cursor {
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

impl Cursor {
    pub fn new(
        start_time: TuioTime,
        session_id: u32,
        cursor_id: u32,
        position: Point2D<f32>,
        velocity: Vector2D<f32>,
        acceleration: f32,
    ) -> Self {
        Self {
            state: TuioState::Added,
            current_time: start_time.clone(),
            start_time,
            session_id,
            cursor_id,
            position,
            velocity,
            speed: velocity.length(),
            acceleration,
        }
    }

    pub fn update(
        &mut self,
        current_time: TuioTime,
        position: Point2D<f32>,
        velocity: Vector2D<f32>,
        acceleration: f32,
    ) {
        self.update_translation(current_time, position, velocity, acceleration);
    }
}
