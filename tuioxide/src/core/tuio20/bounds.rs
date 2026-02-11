use tuioxide_macros::profile;

use crate::core::{container::Container, tuio_time::TuioTime};

pub struct Bounds {
    container: Container,
    bounds: BoundsProfile,
}

impl Bounds {
    pub fn new(start_time: &TuioTime, bounds: BoundsProfile) -> Self {
        let container = Container::new(start_time);
        Self { container, bounds }
    }

    pub fn update(&mut self, time: &TuioTime, bounds: &BoundsProfile) {
        self.container.update(time);
        self.bounds = *bounds;
    }
}

#[derive(Debug, Clone, Copy)]
#[profile("/tuio2/bnd")]
pub struct BoundsProfile {
    session_id: i32,
    position_x: f32,
    position_y: f32,
    angle: f32,
    width: f32,
    height: f32,
    area: f32,
    velocity_x: Option<f32>,
    velocity_y: Option<f32>,
    angle_speed: Option<f32>,
    acceleration: Option<f32>,
    rotation_acceleration: Option<f32>,
}
