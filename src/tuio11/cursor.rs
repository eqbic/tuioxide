use euclid::default::{Point2D, Vector2D};
use rosc::{OscMessage, OscPacket, OscType};

use crate::{
    common::{
        container::Container,
        errors::TuioError,
        osc_utils::{extract_float, extract_int},
        tuio_time::TuioTime,
    },
    tuio11::profile::Profile,
};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    container: Container,
    cursor: CursorProfile,
}

impl Cursor {
    pub fn new(start_time: &TuioTime, cursor: CursorProfile) -> Self {
        let container = Container::new(start_time);
        Self { container, cursor }
    }

    pub fn update(&mut self, time: &TuioTime, cursor: &CursorProfile) {
        self.container.update(time);
        self.cursor = *cursor;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CursorProfile {
    session_id: i32,
    position: Point2D<f32>,
    velocity: Vector2D<f32>,
    acceleration: f32,
}

impl<'a> TryFrom<&'a OscMessage> for CursorProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let session_id = extract_int(message, 1)?;
        let position = Point2D::new(extract_float(message, 2)?, extract_float(message, 3)?);
        let velocity = Vector2D::new(extract_float(message, 4)?, extract_float(message, 5)?);
        let acceleration = extract_float(message, 6)?;
        let cursor = CursorProfile::new(session_id, position, velocity, acceleration);
        Ok(cursor)
    }
}

impl From<CursorProfile> for OscPacket {
    fn from(val: CursorProfile) -> Self {
        OscPacket::Message(OscMessage {
            addr: CursorProfile::address(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(val.session_id),
                OscType::Float(val.position.x),
                OscType::Float(val.position.y),
                OscType::Float(val.velocity.x),
                OscType::Float(val.velocity.y),
                OscType::Float(val.acceleration),
            ],
        })
    }
}

impl<'a> Profile<'a> for CursorProfile {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio/2Dcur".into()
    }
}

impl CursorProfile {
    pub fn new(
        session_id: i32,
        position: Point2D<f32>,
        velocity: Vector2D<f32>,
        acceleration: f32,
    ) -> Self {
        Self {
            session_id,
            position,
            velocity,
            acceleration,
        }
    }
}
