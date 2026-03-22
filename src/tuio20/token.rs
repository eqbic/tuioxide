use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Rotation, Translation, TuioError, TuioProfile, TuioTime,
    Velocity,
};

/// A TUIO 2.0 token entity, representing a tagged physical object on a surface.
///
/// Tokens are distinguished from pointers by carrying identity information
/// (`type_user_id` and `component_id`) that links them to a known physical
/// object type. They track position, orientation, and optionally velocity and
/// acceleration.
///
/// Token instances are created and updated by the client processor in response
/// to incoming `/tuio2/tok` OSC messages.
#[derive(Debug, Clone, Copy)]
pub struct Token {
    container: Container,
    type_user_id: i32,
    component_id: i32,
    translation: Translation,
    rotation: Rotation,
}

impl Token {
    pub(crate) fn new(start_time: &TuioTime, token: TokenProfile) -> Self {
        let container = Container::new(start_time, token.session_id);
        let translation = Translation::new(
            token.position,
            token.velocity.unwrap_or_default(),
            token.acceleration.unwrap_or_default(),
        );
        let rotation = Rotation::new(
            token.angle,
            token.rotation_speed.unwrap_or_default(),
            token.rotation_acceleration.unwrap_or_default(),
        );
        Self {
            container,
            type_user_id: token.type_user_id,
            component_id: token.component_id,
            translation,
            rotation,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, token: &TokenProfile) {
        self.container.update(time);
        self.translation.update_from_message(
            token.position,
            token.velocity.unwrap_or_default(),
            token.acceleration.unwrap_or_default(),
        );
        self.rotation.update(
            token.angle,
            token.rotation_speed.unwrap_or_default(),
            token.rotation_acceleration.unwrap_or_default(),
        );
        self.type_user_id = token.type_user_id;
        self.component_id = token.component_id;
    }

    /// Returns the time at which this token first appeared in the session.
    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    /// Returns the time at which this token was last updated.
    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    /// Returns the session ID assigned to this token by the TUIO source.
    ///
    /// Session IDs uniquely identify an active entity within a session and
    /// are used to correlate `alive`, `set`, and removal messages.
    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    /// Returns the combined type and user ID of this token.
    ///
    /// The upper 16 bits encode the type ID and the lower 16 bits encode
    /// the user ID, as per the TUIO 2.0 specification.
    pub fn type_user_id(&self) -> i32 {
        self.type_user_id
    }

    /// Returns the component ID of this token.
    ///
    /// The component ID identifies which part or face of a physical object
    /// is in contact with the surface.
    pub fn component_id(&self) -> i32 {
        self.component_id
    }

    /// Returns the current normalized position of this token on the surface.
    ///
    /// Coordinates are in the range `[0.0, 1.0]` relative to the surface dimensions.
    pub fn position(&self) -> Position {
        self.translation.position
    }

    /// Returns the current 2D velocity vector of this token.
    ///
    /// Components are expressed as normalized units per frame.
    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    /// Returns the scalar speed of this token, i.e. the Euclidean magnitude
    /// of its velocity vector.
    pub fn speed(&self) -> f32 {
        self.translation.velocity.speed()
    }

    /// Returns the current orientation angle of this token in radians.
    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    /// Returns the current rotational speed of this token in radians per frame.
    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    /// Returns the current rotational acceleration of this token in radians per frame squared.
    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }
}

/// The raw decoded data from a `/tuio2/tok` OSC message.
///
/// `TokenProfile` is an intermediate representation used during OSC decoding.
/// Velocity, acceleration, rotation speed, and rotation acceleration are all
/// optional because the TUIO 2.0 specification allows senders to omit them
/// when the values are not available.
#[derive(Debug, Clone, Copy)]
pub struct TokenProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    position: Position,
    angle: f32,
    velocity: Option<Velocity>,
    rotation_speed: Option<f32>,
    acceleration: Option<f32>,
    rotation_acceleration: Option<f32>,
}

impl<'a> TryFrom<&'a OscMessage> for TokenProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 0);
        Ok(TokenProfile {
            session_id: args.next_int()?,
            type_user_id: args.next_int()?,
            component_id: args.next_int()?,
            position: Position::new(args.next_float()?, args.next_float()?),
            angle: args.next_float()?,
            velocity: if args.remaining() >= 2 {
                Some(Velocity::new(args.next_float()?, args.next_float()?))
            } else {
                None
            },
            rotation_speed: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
            acceleration: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
            rotation_acceleration: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
        })
    }
}

impl From<TokenProfile> for OscPacket {
    fn from(val: TokenProfile) -> Self {
        let mut args = vec![
            OscType::Int(val.session_id),
            OscType::Int(val.type_user_id),
            OscType::Int(val.component_id),
            OscType::Float(val.position.x),
            OscType::Float(val.position.y),
            OscType::Float(val.angle),
        ];

        if let Some(velocity) = val.velocity {
            args.extend([OscType::Float(velocity.x), OscType::Float(velocity.y)]);
        }

        args.extend(val.rotation_speed.into_iter().map(OscType::Float));
        args.extend(val.acceleration.into_iter().map(OscType::Float));
        args.extend(val.rotation_acceleration.into_iter().map(OscType::Float));

        OscPacket::Message(OscMessage {
            addr: TokenProfile::address(),
            args,
        })
    }
}

impl TuioProfile for TokenProfile {
    /// Returns the OSC address for TUIO 2.0 token messages: `"/tuio2/tok"`.
    fn address() -> String {
        "/tuio2/tok".to_string()
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rosc::{OscMessage, OscPacket, OscType};

    use crate::core::TuioProfile;

    use super::TokenProfile;

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Build a fully-populated `/tuio2/tok` OscMessage (all optional fields present).
    fn make_full_msg(
        session_id: i32,
        type_user_id: i32,
        component_id: i32,
        x: f32,
        y: f32,
        angle: f32,
        vx: f32,
        vy: f32,
        rotation_speed: f32,
        acceleration: f32,
        rotation_acceleration: f32,
    ) -> OscMessage {
        OscMessage {
            addr: "/tuio2/tok".to_string(),
            args: vec![
                OscType::Int(session_id),
                OscType::Int(type_user_id),
                OscType::Int(component_id),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(angle),
                OscType::Float(vx),
                OscType::Float(vy),
                OscType::Float(rotation_speed),
                OscType::Float(acceleration),
                OscType::Float(rotation_acceleration),
            ],
        }
    }

    /// Build a minimal `/tuio2/tok` OscMessage (only the 6 required fields).
    fn make_minimal_msg(
        session_id: i32,
        type_user_id: i32,
        component_id: i32,
        x: f32,
        y: f32,
        angle: f32,
    ) -> OscMessage {
        OscMessage {
            addr: "/tuio2/tok".to_string(),
            args: vec![
                OscType::Int(session_id),
                OscType::Int(type_user_id),
                OscType::Int(component_id),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(angle),
            ],
        }
    }

    // ── TryFrom with full args ────────────────────────────────────────────────

    #[test]
    fn full_decodes_session_id() {
        let msg = make_full_msg(7, 1, 2, 0.5, 0.5, 1.0, 0.1, 0.2, 0.3, 0.4, 0.5);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_eq!(tok.session_id(), 7);
    }

    #[test]
    fn full_decodes_type_user_id() {
        let msg = make_full_msg(1, 42, 3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_eq!(tok.type_user_id, 42);
    }

    #[test]
    fn full_decodes_component_id() {
        let msg = make_full_msg(1, 0, 99, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_eq!(tok.component_id, 99);
    }

    #[test]
    fn full_decodes_position() {
        let msg = make_full_msg(1, 0, 0, 0.25, 0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_relative_eq!(tok.position.x, 0.25);
        assert_relative_eq!(tok.position.y, 0.75);
    }

    #[test]
    fn full_decodes_angle() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 1.256, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_relative_eq!(tok.angle, 1.256);
    }

    #[test]
    fn full_decodes_velocity() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        let v = tok.velocity.expect("velocity should be Some");
        assert_relative_eq!(v.x, 1.5);
        assert_relative_eq!(v.y, 2.5);
    }

    #[test]
    fn full_decodes_acceleration() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 9.81, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        let a = tok.acceleration.expect("acceleration should be Some");
        assert_relative_eq!(a, 9.81);
    }

    #[test]
    fn full_decodes_rotation_speed() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        let rs = tok.rotation_speed.expect("rotation_speed should be Some");
        assert_relative_eq!(rs, 0.7);
    }

    #[test]
    fn full_decodes_rotation_acceleration() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3);
        let tok = TokenProfile::try_from(&msg).unwrap();
        let ra = tok
            .rotation_acceleration
            .expect("rotation_acceleration should be Some");
        assert_relative_eq!(ra, 0.3);
    }

    // ── TryFrom with minimal args (no optional fields) ────────────────────────

    #[test]
    fn minimal_decodes_session_id() {
        let msg = make_minimal_msg(5, 1, 2, 0.3, 0.8, 1.57);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_eq!(tok.session_id(), 5);
    }

    #[test]
    fn minimal_decodes_position() {
        let msg = make_minimal_msg(1, 0, 0, 0.4, 0.6, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_relative_eq!(tok.position.x, 0.4);
        assert_relative_eq!(tok.position.y, 0.6);
    }

    #[test]
    fn minimal_decodes_angle() {
        let msg = make_minimal_msg(1, 0, 0, 0.0, 0.0, 2.71);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert_relative_eq!(tok.angle, 2.71);
    }

    #[test]
    fn minimal_velocity_is_none() {
        let msg = make_minimal_msg(1, 0, 0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert!(
            tok.velocity.is_none(),
            "velocity should be None when not provided"
        );
    }

    #[test]
    fn minimal_acceleration_is_none() {
        let msg = make_minimal_msg(1, 0, 0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert!(tok.acceleration.is_none());
    }

    #[test]
    fn minimal_rotation_speed_is_none() {
        let msg = make_minimal_msg(1, 0, 0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert!(tok.rotation_speed.is_none());
    }

    #[test]
    fn minimal_rotation_acceleration_is_none() {
        let msg = make_minimal_msg(1, 0, 0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert!(tok.rotation_acceleration.is_none());
    }

    // ── Error cases ───────────────────────────────────────────────────────────

    #[test]
    fn empty_message_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/tok".to_string(),
            args: vec![],
        };
        assert!(TokenProfile::try_from(&msg).is_err());
    }

    #[test]
    fn too_few_args_returns_error() {
        // Only 4 args provided — need at least 6.
        let msg = OscMessage {
            addr: "/tuio2/tok".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Int(0),
                OscType::Int(0),
                OscType::Float(0.5),
            ],
        };
        assert!(TokenProfile::try_from(&msg).is_err());
    }

    #[test]
    fn wrong_type_for_session_id_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/tok".to_string(),
            args: vec![
                OscType::Float(1.0), // should be Int
                OscType::Int(0),
                OscType::Int(0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
            ],
        };
        assert!(TokenProfile::try_from(&msg).is_err());
    }

    // ── From<TokenProfile> for OscPacket ─────────────────────────────────────

    #[test]
    fn from_full_produces_message_packet() {
        let msg = make_full_msg(1, 2, 3, 0.1, 0.9, 1.5, 0.3, 0.4, 0.5, 0.6, 0.7);
        let tok = TokenProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(tok);
        assert!(matches!(packet, OscPacket::Message(_)));
    }

    #[test]
    fn from_full_address_is_tok() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.addr, "/tuio2/tok");
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_has_11_args() {
        let msg = make_full_msg(1, 2, 3, 0.1, 0.9, 1.5, 0.3, 0.4, 0.5, 0.6, 0.7);
        let tok = TokenProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.args.len(), 11);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_minimal_has_6_args() {
        let msg = make_minimal_msg(1, 0, 0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.args.len(), 6);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_round_trip_session_id() {
        let msg = make_full_msg(42, 1, 2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.args[0], OscType::Int(42));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_round_trip_position() {
        let msg = make_full_msg(1, 0, 0, 0.3, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.args[3], OscType::Float(0.3));
            assert_eq!(out.args[4], OscType::Float(0.7));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_round_trip_velocity() {
        let msg = make_full_msg(1, 0, 0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let tok = TokenProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.args[6], OscType::Float(1.5));
            assert_eq!(out.args[7], OscType::Float(2.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    // ── Partial optional fields ───────────────────────────────────────────────

    #[test]
    fn only_velocity_provided_produces_8_args() {
        // 6 required + 2 velocity = 8 args; subsequent optional fields omitted.
        let msg = OscMessage {
            addr: "/tuio2/tok".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Int(0),
                OscType::Int(0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(1.0), // vx
                OscType::Float(2.0), // vy
            ],
        };
        let tok = TokenProfile::try_from(&msg).unwrap();
        assert!(tok.velocity.is_some());
        assert!(tok.acceleration.is_none());
        assert!(tok.rotation_speed.is_none());
        assert!(tok.rotation_acceleration.is_none());
        if let OscPacket::Message(out) = OscPacket::from(tok) {
            assert_eq!(out.args.len(), 8);
        }
    }
}
