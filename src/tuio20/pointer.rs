use tuioxide_macros::profile;

use crate::common::{container::Container, tuio_time::TuioTime};

pub struct Pointer {
    container: Container,
    pointer: PointerProfile,
}

impl Pointer {
    pub fn new(start_time: &TuioTime, pointer: PointerProfile) -> Self {
        let container = Container::new(start_time);
        Self { container, pointer }
    }

    pub fn update(&mut self, time: &TuioTime, pointer: &PointerProfile) {
        self.container.update(time);
        self.pointer = *pointer;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[profile("/tuio2/ptr")]
pub struct PointerProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    position_x: f32,
    position_y: f32,
    angle: f32,
    shear: f32,
    radius: f32,
    pressure: f32,
    velocity_x: Option<f32>,
    velocity_y: Option<f32>,
    pressure_speed: Option<f32>,
    acceleration: Option<f32>,
    pressure_acceleration: Option<f32>,
}

#[cfg(test)]
mod tests {
    use rosc::{OscMessage, OscType};

    use super::*;

    #[test]
    fn test_decode() {
        let msg = OscMessage {
            addr: "/tuio2/ptr".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Int(2),
                OscType::Int(3),
                OscType::Float(0.5),
                OscType::Float(0.7),
                OscType::Float(1.2),
                OscType::Float(0.5),
                OscType::Float(0.7),
                OscType::Float(1.2),
                OscType::Float(0.5),
                OscType::Float(0.7),
            ],
        };

        let pointer = match PointerProfile::from_osc_message(&msg) {
            Ok(pointer) => pointer,
            Err(error) => panic!("{:?}", error),
        };

        assert_eq!(pointer.component_id, 3);

        assert_eq!(pointer.to_osc_message(), msg);
    }
}
