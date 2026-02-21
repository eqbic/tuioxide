use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Velocity},
    osc_utils::ArgCursor,
    profile::Profile,
    rotation::Rotation,
    translation::Translation,
    tuio_time::TuioTime,
};

#[derive(Debug, Clone, Copy)]
pub struct Object {
    container: Container,
    class_id: i32,
    translation: Translation,
    rotation: Rotation,
}

impl Object {
    pub(crate) fn new(start_time: &TuioTime, object: ObjectProfile) -> Self {
        let container = Container::new(start_time, object.session_id);
        let translation = Translation::new(object.position, object.velocity, object.acceleration);
        let rotation = Rotation::new(
            object.angle,
            object.rotation_speed,
            object.rotation_acceleration,
        );
        Self {
            container,
            class_id: object.class_id,
            translation,
            rotation,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, object: &ObjectProfile) {
        self.container.update(time);
        self.class_id = object.class_id;
        self.translation
            .update(object.position, object.velocity, object.acceleration);
        self.rotation.update(
            object.angle,
            object.rotation_speed,
            object.rotation_acceleration,
        );
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

    pub fn class_id(&self) -> i32 {
        self.class_id
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ObjectProfile {
    session_id: i32,
    class_id: i32,
    position: Position,
    velocity: Velocity,
    acceleration: f32,
    angle: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
}

impl<'a> TryFrom<&'a OscMessage> for ObjectProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 1);
        let session_id = args.next_int()?;
        let class_id = args.next_int()?;
        let position = Position::new(args.next_float()?, args.next_float()?);
        let angle = args.next_float()?;
        let velocity = Velocity::new(args.next_float()?, args.next_float()?);
        let rotation_speed = args.next_float()?;
        let acceleration = args.next_float()?;
        let rotation_acceleration = args.next_float()?;
        let object = ObjectProfile::new(
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

impl From<ObjectProfile> for OscPacket {
    fn from(val: ObjectProfile) -> Self {
        OscPacket::Message(OscMessage {
            addr: ObjectProfile::address(),
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

impl Profile for ObjectProfile {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio/2Dobj".into()
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

impl ObjectProfile {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        session_id: i32,
        class_id: i32,
        position: Position,
        angle: f32,
        velocity: Velocity,
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
