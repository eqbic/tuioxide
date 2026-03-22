use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Translation, TuioError, TuioProfile, TuioTime, Velocity,
};

/// A TUIO 1.1 cursor, representing a single touch point on a surface.
///
/// A `Cursor` tracks the position, velocity, and acceleration of a finger or
/// pointer contact. It corresponds to the `/tuio/2Dcur` OSC address and is
/// the most common entity type in TUIO 1.1 sessions.
///
/// Cursors are created and updated by the TUIO 1.1 [`Processor`](crate::client::tuio11::processor::Processor)
/// and surfaced to the application via [`CursorEvent`](crate::tuio11::event::CursorEvent).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cursor {
    container: Container,
    translation: Translation,
}

impl TuioProfile for Cursor {
    fn address() -> String {
        "/tuio/2Dcur".into()
    }

    fn session_id(&self) -> i32 {
        self.container.session_id
    }
}

impl<'a> TryFrom<&'a OscMessage> for Cursor {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 1);
        let session_id = args.next_int()?;
        let position = Position::new(args.next_float()?, args.next_float()?);
        let velocity = Velocity::new(args.next_float()?, args.next_float()?);
        let acceleration = args.next_float()?;

        let container = Container::new(&TuioTime::from_system_time().unwrap(), session_id);
        let translation = Translation::new(position, velocity, acceleration);
        Ok(Cursor {
            container,
            translation,
        })
    }
}

impl From<Cursor> for OscPacket {
    fn from(cursor: Cursor) -> Self {
        OscPacket::Message(OscMessage {
            addr: Cursor::address(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(cursor.session_id()),
                OscType::Float(cursor.position().x),
                OscType::Float(cursor.position().y),
                OscType::Float(cursor.velocity().x),
                OscType::Float(cursor.velocity().y),
                OscType::Float(cursor.acceleration()),
            ],
        })
    }
}

impl Cursor {
    pub fn new(session_id: i32, position: Position, velocity: Velocity, acceleration: f32) -> Self {
        let container = Container::new(&TuioTime::from_system_time().unwrap(), session_id);
        let translation = Translation::new(position, velocity, acceleration);
        Self {
            container,
            translation,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, cursor: &Cursor) {
        self.container.update(time);
        self.translation = cursor.translation;
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

    pub fn set_position(&mut self, position: Position) {
        self.translation.position = position;
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

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;
    use rosc::{OscMessage, OscPacket, OscType};

    use crate::core::{Position, Velocity};

    use super::Cursor;

    // ── Helpers ──────────────────────────────────────────────────────────────

    /// Build a well-formed `/tuio/2Dcur` "set" OscMessage with 7 args.
    fn make_set_msg(session_id: i32, x: f32, y: f32, vx: f32, vy: f32, accel: f32) -> OscMessage {
        OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Int(session_id),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(vx),
                OscType::Float(vy),
                OscType::Float(accel),
            ],
        }
    }

    // ── TryFrom<&OscMessage> ─────────────────────────────────────────────────

    #[test]
    fn try_from_decodes_session_id() {
        let msg = make_set_msg(7, 0.1, 0.2, 0.3, 0.4, 0.5);
        let cursor = Cursor::try_from(&msg).unwrap();
        assert_eq!(cursor.session_id(), 7);
    }

    #[test]
    fn try_from_decodes_position() {
        let msg = make_set_msg(1, 0.25, 0.75, 0.0, 0.0, 0.0);
        let cursor = Cursor::try_from(&msg).unwrap();
        assert_relative_eq!(cursor.position().x, 0.25);
        assert_relative_eq!(cursor.position().y, 0.75);
    }

    #[test]
    fn try_from_decodes_velocity() {
        let msg = make_set_msg(1, 0.0, 0.0, 1.5, 2.5, 0.0);
        let cursor = Cursor::try_from(&msg).unwrap();
        assert_relative_eq!(cursor.velocity().x, 1.5);
        assert_relative_eq!(cursor.velocity().y, 2.5);
    }

    #[test]
    fn try_from_decodes_acceleration() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 2.34);
        let cursor = Cursor::try_from(&msg).unwrap();
        assert_relative_eq!(cursor.acceleration(), 2.34);
    }

    #[test]
    fn try_from_missing_args_returns_error() {
        // Only 3 args (instead of 7) — should fail with MissingArguments.
        let msg = OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Int(1),
                OscType::Float(0.5),
            ],
        };
        assert!(Cursor::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_wrong_type_returns_error() {
        // session_id position is occupied by a Float instead of Int.
        let msg = OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Float(1.0), // should be Int
                OscType::Float(0.1),
                OscType::Float(0.2),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
            ],
        };
        assert!(Cursor::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_empty_message_returns_error() {
        let msg = OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![],
        };
        assert!(Cursor::try_from(&msg).is_err());
    }

    // ── Round-trip: From<Cursor> for OscPacket ────────────────────────────────

    #[test]
    fn round_trip_session_id() {
        let msg = make_set_msg(42, 0.1, 0.9, 0.5, 0.3, 1.2);
        let cursor = Cursor::try_from(&msg).unwrap();
        let packet = OscPacket::from(cursor);
        if let OscPacket::Message(out) = packet {
            assert_eq!(out.args[1], OscType::Int(42));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_position() {
        let msg = make_set_msg(1, 0.3, 0.7, 0.0, 0.0, 0.0);
        let cursor = Cursor::try_from(&msg).unwrap();
        let packet = OscPacket::from(cursor);
        if let OscPacket::Message(out) = packet {
            assert_eq!(out.args[2], OscType::Float(0.3));
            assert_eq!(out.args[3], OscType::Float(0.7));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_velocity() {
        let msg = make_set_msg(1, 0.0, 0.0, 1.5, 2.5, 0.0);
        let cursor = Cursor::try_from(&msg).unwrap();
        let packet = OscPacket::from(cursor);
        if let OscPacket::Message(out) = packet {
            assert_eq!(out.args[4], OscType::Float(1.5));
            assert_eq!(out.args[5], OscType::Float(2.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_acceleration() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 9.81);
        let cursor = Cursor::try_from(&msg).unwrap();
        let packet = OscPacket::from(cursor);
        if let OscPacket::Message(out) = packet {
            assert_eq!(out.args[6], OscType::Float(9.81));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_address_is_2dcur() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0);
        let cursor = Cursor::try_from(&msg).unwrap();
        let packet = OscPacket::from(cursor);
        if let OscPacket::Message(out) = packet {
            assert_eq!(out.addr, "/tuio/2Dcur");
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_first_arg_is_set() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0);
        let cursor = Cursor::try_from(&msg).unwrap();
        let packet = OscPacket::from(cursor);
        if let OscPacket::Message(out) = packet {
            assert_eq!(out.args[0], OscType::String("set".to_string()));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    // ── Cursor::new ───────────────────────────────────────────────────────────

    #[test]
    fn new_stores_session_id() {
        let c = Cursor::new(99, Position::new(0.0, 0.0), Velocity::new(0.0, 0.0), 0.0);
        assert_eq!(c.session_id(), 99);
    }

    #[test]
    fn new_stores_position() {
        let c = Cursor::new(1, Position::new(0.4, 0.6), Velocity::new(0.0, 0.0), 0.0);
        assert_relative_eq!(c.position().x, 0.4);
        assert_relative_eq!(c.position().y, 0.6);
    }

    #[test]
    fn new_stores_velocity() {
        let c = Cursor::new(1, Position::new(0.0, 0.0), Velocity::new(1.0, 2.0), 0.0);
        assert_relative_eq!(c.velocity().x, 1.0);
        assert_relative_eq!(c.velocity().y, 2.0);
    }

    #[test]
    fn new_stores_acceleration() {
        let c = Cursor::new(1, Position::new(0.0, 0.0), Velocity::new(0.0, 0.0), 5.5);
        assert_relative_eq!(c.acceleration(), 5.5);
    }

    #[test]
    fn new_start_time_equals_current_time_initially() {
        let c = Cursor::new(1, Position::new(0.0, 0.0), Velocity::new(0.0, 0.0), 0.0);
        assert_eq!(c.start_time(), c.current_time());
    }
}
