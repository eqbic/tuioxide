use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Size, Velocity},
    osc_utils::{extract_float, extract_int},
    tuio_time::TuioTime,
    tuio11::profile::Profile,
};

#[derive(Debug, Clone, Copy)]
pub struct Blob {
    container: Container,
    angle: f32,
    size: Size,
    area: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
}

impl Blob {
    pub fn new(start_time: &TuioTime, blob: BlobProfile) -> Self {
        let container = Container::new(start_time, blob.session_id, blob.position);
        Self {
            container,
            angle: blob.angle,
            size: blob.size,
            area: blob.area,
            rotation_speed: blob.rotation_speed,
            rotation_acceleration: blob.rotation_acceleration,
        }
    }

    pub fn update(&mut self, time: &TuioTime, blob: &BlobProfile) {
        self.container.update(time, blob);
        self.angle = blob.angle;
        self.size = blob.size;
        self.area = blob.area;
        self.rotation_speed = blob.rotation_speed;
        self.rotation_acceleration = blob.rotation_acceleration;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlobProfile {
    session_id: i32,
    position: Position,
    velocity: Velocity,
    acceleration: f32,
    angle: f32,
    size: Size,
    area: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
}

impl<'a> TryFrom<&'a OscMessage> for BlobProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let session_id = extract_int(message, 1)?;
        let position = Position::new(extract_float(message, 2)?, extract_float(message, 3)?);
        let velocity = Velocity::new(extract_float(message, 4)?, extract_float(message, 5)?);
        let acceleration = extract_float(message, 6)?;
        let angle = extract_float(message, 7)?;
        let size = Size::new(extract_float(message, 8)?, extract_float(message, 9)?);
        let area = extract_float(message, 10)?;
        let rotation_speed = extract_float(message, 11)?;
        let rotation_acceleration = extract_float(message, 12)?;
        let blob = BlobProfile::new(
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

impl From<BlobProfile> for OscPacket {
    fn from(val: BlobProfile) -> Self {
        OscPacket::Message(OscMessage {
            addr: BlobProfile::address(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(val.session_id),
                OscType::Float(val.position.x),
                OscType::Float(val.position.y),
                OscType::Float(val.angle),
                OscType::Float(val.size.x),
                OscType::Float(val.size.y),
                OscType::Float(val.area),
                OscType::Float(val.velocity.x),
                OscType::Float(val.velocity.y),
                OscType::Float(val.rotation_speed),
                OscType::Float(val.acceleration),
                OscType::Float(val.rotation_acceleration),
            ],
        })
    }
}

impl Profile for BlobProfile {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio/2Dblb".into()
    }

    fn position(&self) -> Position {
        self.position
    }

    fn velocity(&self) -> Velocity {
        self.velocity
    }

    fn acceleration(&self) -> f32 {
        self.acceleration
    }
}

impl BlobProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_id: i32,
        position: Position,
        angle: f32,
        size: Size,
        area: f32,
        velocity: Velocity,
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
