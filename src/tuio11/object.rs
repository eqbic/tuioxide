use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Rotation, Translation, TuioError, TuioProfile, TuioTime,
    Velocity,
};

/// A TUIO 1.1 tangible object tracked on a surface (`/tuio/2Dobj`).
///
/// An `Object` represents a physical tangible that can be
/// placed and moved on a TUIO-enabled surface. Each object carries a
/// [`class_id`](Object::class_id) that identifies the fiducial marker pattern,
/// as well as full 2D position, velocity, acceleration, angle, rotation speed,
/// and rotation acceleration data.
///
/// Instances are created and updated internally by the TUIO 1.1 processor and
/// surfaced to the application via [`ObjectEvent`](crate::tuio11::event::ObjectEvent).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Object {
    container: Container,
    class_id: i32,
    translation: Translation,
    rotation: Rotation,
}

impl<'a> TryFrom<&'a OscMessage> for Object {
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
        let container = Container::new(&TuioTime::from_system_time().unwrap(), session_id);
        let translation = Translation::new(position, velocity, acceleration);
        let rotation = Rotation::new(angle, rotation_speed, rotation_acceleration);

        Ok(Object {
            container,
            class_id,
            translation,
            rotation,
        })
    }
}

impl From<Object> for OscPacket {
    fn from(object: Object) -> Self {
        OscPacket::Message(OscMessage {
            addr: Object::address(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(object.session_id()),
                OscType::Int(object.class_id()),
                OscType::Float(object.position().x),
                OscType::Float(object.position().y),
                OscType::Float(object.angle()),
                OscType::Float(object.velocity().x),
                OscType::Float(object.velocity().y),
                OscType::Float(object.rotation_speed()),
                OscType::Float(object.acceleration()),
                OscType::Float(object.rotation_acceleration()),
            ],
        })
    }
}

impl TuioProfile for Object {
    fn address() -> String {
        "/tuio/2Dobj".into()
    }

    fn session_id(&self) -> i32 {
        self.session_id()
    }
}

impl Object {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_id: i32,
        class_id: i32,
        position: Position,
        velocity: Velocity,
        acceleration: f32,
        angle: f32,
        rotation_speed: f32,
        rotation_acceleration: f32,
    ) -> Self {
        let container = Container::new(&TuioTime::from_system_time().unwrap(), session_id);
        let translation = Translation::new(position, velocity, acceleration);
        let rotation = Rotation::new(angle, rotation_speed, rotation_acceleration);
        Self {
            container,
            class_id,
            translation,
            rotation,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, object: &Object) {
        self.container.update(time);
        self.class_id = object.class_id;
        self.translation = object.translation;
        self.rotation = object.rotation;
    }

    /// Returns the timestamp of the most recent update for this object.
    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    /// Returns the timestamp at which this object first appeared on the surface.
    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    /// Returns the session ID assigned to this object by the TUIO source.
    ///
    /// Session IDs uniquely identify active entities within a session and are
    /// reassigned when an entity disappears and reappears.
    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    /// Returns the class ID (fiducial marker ID) of this object.
    ///
    /// The class ID identifies the physical marker pattern placed on the surface
    /// and remains stable across multiple sessions for the same physical object.
    pub fn class_id(&self) -> i32 {
        self.class_id
    }

    /// Returns the current normalized 2D position of this object on the surface.
    ///
    /// Coordinates are in the range `[0.0, 1.0]` relative to the surface dimensions.
    pub fn position(&self) -> Position {
        self.translation.position
    }

    /// Returns the current 2D velocity vector of this object.
    ///
    /// Components represent the rate of change of the normalized position per frame.
    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    /// Returns the current translational acceleration scalar of this object.
    ///
    /// Positive values indicate speeding up; negative values indicate slowing down.
    pub fn acceleration(&self) -> f32 {
        self.translation.acceleration
    }

    /// Returns the current rotation angle of this object in radians.
    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    /// Returns the current rotational speed of this object in radians per frame.
    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    /// Returns the current rotational acceleration of this object in radians per frame².
    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;
    use rosc::{OscMessage, OscPacket, OscType};

    use crate::core::{Position, Velocity};

    use super::Object;

    // ── Helpers ──────────────────────────────────────────────────────────────

    /// Build a well-formed `/tuio/2Dobj` "set" OscMessage with 11 args.
    #[allow(clippy::too_many_arguments)]
    fn make_set_msg(
        session_id: i32,
        class_id: i32,
        x: f32,
        y: f32,
        angle: f32,
        vx: f32,
        vy: f32,
        rot_speed: f32,
        accel: f32,
        rot_accel: f32,
    ) -> OscMessage {
        OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Int(session_id),
                OscType::Int(class_id),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(angle),
                OscType::Float(vx),
                OscType::Float(vy),
                OscType::Float(rot_speed),
                OscType::Float(accel),
                OscType::Float(rot_accel),
            ],
        }
    }

    fn default_msg() -> OscMessage {
        make_set_msg(5, 2, 0.1, 0.9, 1.57, 0.3, 0.4, 0.2, 0.5, 0.1)
    }

    // ── TryFrom<&OscMessage> ─────────────────────────────────────────────────

    #[test]
    fn try_from_decodes_session_id() {
        let msg = make_set_msg(13, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_eq!(obj.session_id(), 13);
    }

    #[test]
    fn try_from_decodes_class_id() {
        let msg = make_set_msg(1, 7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_eq!(obj.class_id(), 7);
    }

    #[test]
    fn try_from_decodes_position() {
        let msg = make_set_msg(1, 0, 0.25, 0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_relative_eq!(obj.position().x, 0.25);
        assert_relative_eq!(obj.position().y, 0.75);
    }

    #[test]
    fn try_from_decodes_angle() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 2.56, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_relative_eq!(obj.angle(), 2.56);
    }

    #[test]
    fn try_from_decodes_velocity() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_relative_eq!(obj.velocity().x, 1.5);
        assert_relative_eq!(obj.velocity().y, 2.5);
    }

    #[test]
    fn try_from_decodes_rotation_speed() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_relative_eq!(obj.rotation_speed(), 0.8);
    }

    #[test]
    fn try_from_decodes_acceleration() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 9.81, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        assert_relative_eq!(obj.acceleration(), 9.81);
    }

    #[test]
    fn try_from_decodes_rotation_acceleration() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.2);
        let obj = Object::try_from(&msg).unwrap();
        assert_relative_eq!(obj.rotation_acceleration(), 2.2);
    }

    #[test]
    fn try_from_missing_args_returns_error() {
        // Only 5 args instead of 11.
        let msg = OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Int(1),
                OscType::Int(2),
                OscType::Float(0.1),
                OscType::Float(0.2),
            ],
        };
        assert!(Object::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_wrong_type_for_session_id_returns_error() {
        // Session ID should be Int, not Float.
        let msg = OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Float(1.0), // should be Int
                OscType::Int(0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
            ],
        };
        assert!(Object::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_wrong_type_for_class_id_returns_error() {
        // Class ID should be Int, not Float.
        let msg = OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Int(1),
                OscType::Float(0.0), // should be Int
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
            ],
        };
        assert!(Object::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_empty_message_returns_error() {
        let msg = OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![],
        };
        assert!(Object::try_from(&msg).is_err());
    }

    // ── Round-trip: From<Object> for OscPacket ────────────────────────────────

    #[test]
    fn round_trip_produces_message_packet() {
        let obj = Object::try_from(&default_msg()).unwrap();
        let packet = OscPacket::from(obj);
        assert!(matches!(packet, OscPacket::Message(_)));
    }

    #[test]
    fn round_trip_address_is_2dobj() {
        let obj = Object::try_from(&default_msg()).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.addr, "/tuio/2Dobj");
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_first_arg_is_set() {
        let obj = Object::try_from(&default_msg()).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[0], OscType::String("set".to_string()));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_session_id() {
        let msg = make_set_msg(99, 3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[1], OscType::Int(99));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_class_id() {
        let msg = make_set_msg(1, 15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[2], OscType::Int(15));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_position() {
        let msg = make_set_msg(1, 0, 0.4, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[3], OscType::Float(0.4));
            assert_eq!(out.args[4], OscType::Float(0.6));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_angle() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 1.57, 0.0, 0.0, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[5], OscType::Float(1.57));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_velocity() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[6], OscType::Float(1.5));
            assert_eq!(out.args[7], OscType::Float(2.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_rotation_speed() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.3, 0.0, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[8], OscType::Float(3.3));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_acceleration() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.4, 0.0);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[9], OscType::Float(4.4));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_rotation_acceleration() {
        let msg = make_set_msg(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.5);
        let obj = Object::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args[10], OscType::Float(5.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn round_trip_has_11_args() {
        let obj = Object::try_from(&default_msg()).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(obj) {
            assert_eq!(out.args.len(), 11);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    // ── Object::new ───────────────────────────────────────────────────────────

    #[test]
    fn new_stores_all_fields() {
        let obj = Object::new(
            10,
            3,
            Position::new(0.1, 0.2),
            Velocity::new(0.5, 0.6),
            1.1,
            2.2,
            3.3,
            4.4,
        );
        assert_eq!(obj.session_id(), 10);
        assert_eq!(obj.class_id(), 3);
        assert_relative_eq!(obj.position().x, 0.1);
        assert_relative_eq!(obj.position().y, 0.2);
        assert_relative_eq!(obj.velocity().x, 0.5);
        assert_relative_eq!(obj.velocity().y, 0.6);
        assert_relative_eq!(obj.acceleration(), 1.1);
        assert_relative_eq!(obj.angle(), 2.2);
        assert_relative_eq!(obj.rotation_speed(), 3.3);
        assert_relative_eq!(obj.rotation_acceleration(), 4.4);
    }

    #[test]
    fn new_start_time_equals_current_time_initially() {
        let obj = Object::new(
            1,
            0,
            Position::new(0.0, 0.0),
            Velocity::new(0.0, 0.0),
            0.0,
            0.0,
            0.0,
            0.0,
        );
        assert_eq!(obj.start_time(), obj.current_time());
    }
}
