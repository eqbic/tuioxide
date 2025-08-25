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
pub struct Object {
    session_id: i32,
    class_id: i32,
    position: Point2D<f32>,
    velocity: Vector2D<f32>,
    acceleration: f32,
    angle: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
}

impl<'a> TryFrom<&'a OscMessage> for Object {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let session_id = extract_int(message, 1)?;
        let class_id = extract_int(message, 2)?;
        let position = Point2D::new(extract_float(message, 3)?, extract_float(message, 4)?);
        let angle = extract_float(message, 5)?;
        let velocity = Vector2D::new(extract_float(message, 6)?, extract_float(message, 7)?);
        let rotation_speed = extract_float(message, 8)?;
        let acceleration = extract_float(message, 9)?;
        let rotation_acceleration = extract_float(message, 10)?;
        let object = Object::new(
            session_id,
            class_id,
            position,
            angle,
            velocity,
            rotation_speed,
            acceleration,
            rotation_acceleration,
        );
        Ok(object)
    }
}

impl From<Object> for OscPacket {
    fn from(val: Object) -> Self {
        OscPacket::Message(OscMessage {
            addr: "/tuio/2Dobj".into(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(val.session_id),
                OscType::Int(val.class_id),
                OscType::Float(val.position.x),
                OscType::Float(val.position.y),
                OscType::Float(val.angle),
                OscType::Float(val.velocity.x),
                OscType::Float(val.velocity.y),
                OscType::Float(val.rotation_speed),
                OscType::Float(val.acceleration),
                OscType::Float(val.rotation_acceleration),
            ],
        })
    }
}

impl<'a> Profile<'a> for Object {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio/2Dobj".into()
    }
}

impl Object {
    pub fn new(
        session_id: i32,
        class_id: i32,
        position: Point2D<f32>,
        angle: f32,
        velocity: Vector2D<f32>,
        rotation_speed: f32,
        acceleration: f32,
        rotation_acceleration: f32,
    ) -> Self {
        Self {
            session_id,
            class_id,
            position,
            velocity,
            acceleration,
            angle,
            rotation_acceleration,
            rotation_speed,
        }
    }
}
