use tuioxide_macros::profile;

use crate::common::{container::Container, tuio_time::TuioTime};

#[derive(Debug, Clone, Copy)]
pub struct Token {
    container: Container,
    token: TokenProfile,
}

impl Token {
    pub fn new(start_time: &TuioTime, token: TokenProfile) -> Self {
        let container = Container::new(start_time);
        Self { container, token }
    }

    pub fn update(&mut self, time: &TuioTime, token: &TokenProfile) {
        self.container.update(time);
        self.token = *token;
    }
}

#[derive(Debug, Clone, Copy)]
#[profile("/tuio2/tok")]
pub struct TokenProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    position_x: f32,
    position_y: f32,
    angle: f32,
    velocity_x: Option<f32>,
    velocity_y: Option<f32>,
    angle_speed: Option<f32>,
    acceleration: Option<f32>,
    rotation_acceleration: Option<f32>,
}

impl TokenProfile {
    pub fn session_id(&self) -> i32 {
        self.session_id
    }
}
