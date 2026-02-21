use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Velocity},
    osc_utils::ArgCursor,
    profile::Profile,
    tuio_time::TuioTime,
};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    container: Container,
}

impl Cursor {
    pub fn new(start_time: &TuioTime, cursor: CursorProfile) -> Self {
        let container = Container::new(start_time, cursor.session_id, cursor.position);
        Self { container }
    }

    pub fn update(&mut self, time: &TuioTime, cursor: &CursorProfile) {
        self.container.update(time, cursor);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CursorProfile {
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
    pub fn new(session_id: i32, position: Position, velocity: Velocity, acceleration: f32) -> Self {
        Self {
            session_id,
            position,
            velocity,
            acceleration,
        }
    }
}
