use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Size, Velocity},
    osc_utils::ArgCursor,
    profile::Profile,
    rotation::Rotation,
    translation::Translation,
    tuio_time::TuioTime,
};

#[derive(Debug, Clone, Copy)]
pub struct Blob {
    container: Container,
    translation: Translation,
    rotation: Rotation,
    size: Size,
    area: f32,
}

impl Blob {
    pub(crate) fn new(start_time: &TuioTime, blob: BlobProfile) -> Self {
        let container = Container::new(start_time, blob.session_id);
        let translation = Translation::new(blob.position, blob.velocity, blob.acceleration);
        let rotation = Rotation::new(blob.angle, blob.rotation_speed, blob.rotation_acceleration);
        Self {
            container,
            translation,
            rotation,
            size: blob.size,
            area: blob.area,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, blob: &BlobProfile) {
        self.container.update(time);
        self.translation
            .update(blob.position, blob.velocity, blob.acceleration);
        self.rotation
            .update(blob.angle, blob.rotation_speed, blob.rotation_acceleration);
        self.size = blob.size;
        self.area = blob.area;
    }

    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    pub fn position(&self) -> Position {
        self.translation.position
    }

    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    pub fn acceleration(&self) -> f32 {
        self.translation.acceleration
    }

    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn area(&self) -> f32 {
        self.area
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BlobProfile {
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
        let mut args = ArgCursor::new(message, 1);
        let session_id = args.next_int()?;
        let position = Position::new(args.next_float()?, args.next_float()?);
        let velocity = Velocity::new(args.next_float()?, args.next_float()?);
        let acceleration = args.next_float()?;
        let angle = args.next_float()?;
        let size = Size::new(args.next_float()?, args.next_float()?);
        let area = args.next_float()?;
        let rotation_speed = args.next_float()?;
        let rotation_acceleration = args.next_float()?;
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
                OscType::Float(val.size.width),
                OscType::Float(val.size.height),
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
}

impl BlobProfile {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
