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

/// A TUIO 1.1 cursor, representing a single touch point on a surface.
///
/// A `Cursor` tracks the position, velocity, and acceleration of a finger or
/// pointer contact. It corresponds to the `/tuio/2Dcur` OSC address and is
/// the most common entity type in TUIO 1.1 sessions.
///
/// Cursors are created and updated by the TUIO 1.1 [`Processor`](crate::client::tuio11::processor::Processor)
/// and surfaced to the application via [`CursorEvent`](crate::core::tuio11::event::CursorEvent).
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

    /// Returns the [`TuioTime`] at which this cursor was last updated.
    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    /// Returns the [`TuioTime`] at which this cursor first appeared.
    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    /// Returns the session ID uniquely identifying this cursor within the current session.
    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    /// Returns the current normalized position of this cursor on the surface.
    ///
    /// Coordinates are in the range `[0.0, 1.0]`, where `(0, 0)` is the top-left
    /// corner and `(1, 1)` is the bottom-right corner.
    pub fn position(&self) -> Position {
        self.translation.position
    }

    /// Returns the current 2D velocity vector of this cursor.
    ///
    /// Components are expressed as normalized units per frame.
    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    /// Returns the scalar motion acceleration of this cursor.
    ///
    /// Positive values indicate the cursor is speeding up; negative values
    /// indicate it is slowing down.
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
