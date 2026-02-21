use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Velocity},
    osc_utils::ArgCursor,
    profile::Profile,
    translation::Translation,
    tuio_time::TuioTime,
};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    container: Container,
    translation: Translation,
}

impl Cursor {
    pub(crate) fn new(start_time: &TuioTime, cursor: CursorProfile) -> Self {
        let container = Container::new(start_time, cursor.session_id);
        let translation = Translation::new(cursor.position, cursor.velocity, cursor.acceleration);
        Self {
            container,
            translation,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, cursor: &CursorProfile) {
        self.container.update(time);
        self.translation
            .update(cursor.position, cursor.velocity, cursor.acceleration);
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CursorProfile {
    session_id: i32,
    position: Position,
    velocity: Velocity,
    acceleration: f32,
}

impl<'a> TryFrom<&'a OscMessage> for CursorProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 1);
        let session_id = args.next_int()?;
        let position = Position::new(args.next_float()?, args.next_float()?);
        let velocity = Velocity::new(args.next_float()?, args.next_float()?);
        let acceleration = args.next_float()?;
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

impl Profile for CursorProfile {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio/2Dcur".into()
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

impl CursorProfile {
    pub(crate) fn new(
        session_id: i32,
        position: Position,
        velocity: Velocity,
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
