use euclid::default::{Point2D, Vector2D};
use rosc::{OscMessage, OscPacket, OscType};

use crate::{
    common::{
        errors::TuioError,
        osc_utils::{extract_float, extract_int},
    },
    tuio11::profile::Profile,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Blob {
    session_id: i32,
    position: Point2D<f32>,
    velocity: Vector2D<f32>,
    acceleration: f32,
    angle: f32,
    size: Vector2D<f32>,
    area: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
}

impl<'a> TryFrom<&'a OscMessage> for Blob {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let session_id = extract_int(&message, 1)?;
        let position = Point2D::new(extract_float(&message, 2)?, extract_float(&message, 3)?);
        let velocity = Vector2D::new(extract_float(&message, 4)?, extract_float(&message, 5)?);
        let acceleration = extract_float(&message, 6)?;
        let angle = extract_float(&message, 7)?;
        let size = Vector2D::new(extract_float(&message, 8)?, extract_float(&message, 9)?);
        let area = extract_float(&message, 10)?;
        let rotation_speed = extract_float(&message, 11)?;
        let rotation_acceleration = extract_float(&message, 12)?;
        let blob = Blob::new(
            session_id,
            position,
            angle,
            size,
            area,
            velocity,
            rotation_speed,
            acceleration,
            rotation_acceleration,
        );
        Ok(blob)
    }
}

impl Into<OscPacket> for Blob {
    fn into(self) -> OscPacket {
        OscPacket::Message(OscMessage {
            addr: "/tuio/2Dblb".into(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(self.session_id),
                OscType::Float(self.position.x),
                OscType::Float(self.position.y),
                OscType::Float(self.angle),
                OscType::Float(self.size.x),
                OscType::Float(self.size.y),
                OscType::Float(self.area),
                OscType::Float(self.velocity.x),
                OscType::Float(self.velocity.y),
                OscType::Float(self.rotation_speed),
                OscType::Float(self.acceleration),
                OscType::Float(self.rotation_acceleration),
            ],
        })
    }
}

impl<'a> Profile<'a> for Blob {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio/2Dblb".into()
    }
}

impl Blob {
    pub fn new(
        session_id: i32,
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
            session_id,
            position,
            velocity,
            acceleration,
            angle,
            size,
            area,
            rotation_acceleration,
            rotation_speed,
        }
    }
}
