use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Rotation, Size, Translation, TuioError, TuioProfile, TuioTime,
    Velocity,
};

/// A TUIO 2.0 bounds entity, representing the bounding rectangle of a tangible
/// object on the surface.
///
/// A `Bounds` tracks positional and rotational state over time, including an
/// optional velocity, rotation speed, and their respective accelerations.
/// It corresponds to the `/tuio2/bnd` OSC address.
#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    container: Container,
    translation: Translation,
    rotation: Rotation,
    size: Size,
    area: f32,
}

impl Bounds {
    pub(crate) fn new(start_time: &TuioTime, bounds: BoundsProfile) -> Self {
        let container = Container::new(start_time, bounds.session_id);
        let translation = Translation::new(
            bounds.position,
            bounds.velocity.unwrap_or_default(),
            bounds.acceleration.unwrap_or_default(),
        );
        let rotation = Rotation::new(
            bounds.angle,
            bounds.rotation_speed.unwrap_or_default(),
            bounds.rotation_acceleration.unwrap_or_default(),
        );
        Self {
            container,
            size: bounds.size,
            area: bounds.area,
            translation,
            rotation,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, bounds: &BoundsProfile) {
        self.container.update(time);
        self.translation.update_from_message(
            bounds.position,
            bounds.velocity.unwrap_or_default(),
            bounds.acceleration.unwrap_or_default(),
        );
        self.rotation.update(
            bounds.angle,
            bounds.rotation_speed.unwrap_or_default(),
            bounds.rotation_acceleration.unwrap_or_default(),
        );

        self.size = bounds.size;
        self.area = bounds.area;
    }

    /// Returns the [`TuioTime`] at which this bounds entity first appeared.
    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    /// Returns the [`TuioTime`] of the most recent update to this bounds entity.
    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    /// Returns the session ID uniquely identifying this bounds entity within the
    /// current TUIO session.
    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    /// Returns the current normalized position of the bounding rectangle's center.
    pub fn position(&self) -> Position {
        self.translation.position
    }

    /// Returns the current 2D velocity vector of this bounds entity.
    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    /// Returns the scalar speed (Euclidean magnitude of the velocity vector).
    pub fn speed(&self) -> f32 {
        self.translation.velocity.speed()
    }

    /// Returns the current rotation angle of this bounds entity, in radians.
    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    /// Returns the current rotation speed of this bounds entity, in radians per frame.
    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    /// Returns the current rotational acceleration of this bounds entity.
    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }

    /// Returns the normalized size (width and height) of the bounding rectangle.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns the normalized area of the bounding rectangle.
    pub fn area(&self) -> f32 {
        self.area
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BoundsProfile {
    session_id: i32,
    position: Position,
    angle: f32,
    size: Size,
    area: f32,
    velocity: Option<Velocity>,
    rotation_speed: Option<f32>,
    acceleration: Option<f32>,
    rotation_acceleration: Option<f32>,
}

impl<'a> TryFrom<&'a OscMessage> for BoundsProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 0);
        Ok(BoundsProfile {
            session_id: args.next_int()?,
            position: Position::new(args.next_float()?, args.next_float()?),
            angle: args.next_float()?,
            size: Size::new(args.next_float()?, args.next_float()?),
            area: args.next_float()?,
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

impl From<BoundsProfile> for OscPacket {
    fn from(val: BoundsProfile) -> Self {
        let mut args = vec![
            OscType::Int(val.session_id),
            OscType::Float(val.position.x),
            OscType::Float(val.position.y),
            OscType::Float(val.angle),
            OscType::Float(val.size.width),
            OscType::Float(val.size.height),
            OscType::Float(val.area),
        ];

        if let Some(velocity) = val.velocity {
            args.extend([OscType::Float(velocity.x), OscType::Float(velocity.y)]);
        }

        args.extend(val.rotation_speed.into_iter().map(OscType::Float));
        args.extend(val.acceleration.into_iter().map(OscType::Float));
        args.extend(val.rotation_acceleration.into_iter().map(OscType::Float));

        OscPacket::Message(OscMessage {
            addr: BoundsProfile::address(),
            args,
        })
    }
}

impl TuioProfile for BoundsProfile {
    fn address() -> String {
        "/tuio2/bnd".to_string()
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

    use super::BoundsProfile;

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Build a fully-populated `/tuio2/bnd` OscMessage (all optional fields included).
    ///
    /// Decode order:
    ///   [0] session_id, [1] x, [2] y, [3] angle, [4] width, [5] height,
    ///   [6] area, [7] vx, [8] vy, [9] rot_speed, [10] accel, [11] rot_accel
    #[allow(clippy::too_many_arguments)]
    fn make_full_msg(
        session_id: i32,
        x: f32,
        y: f32,
        angle: f32,
        width: f32,
        height: f32,
        area: f32,
        vx: f32,
        vy: f32,
        rot_speed: f32,
        accel: f32,
        rot_accel: f32,
    ) -> OscMessage {
        OscMessage {
            addr: "/tuio2/bnd".to_string(),
            args: vec![
                OscType::Int(session_id),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(angle),
                OscType::Float(width),
                OscType::Float(height),
                OscType::Float(area),
                OscType::Float(vx),
                OscType::Float(vy),
                OscType::Float(rot_speed),
                OscType::Float(accel),
                OscType::Float(rot_accel),
            ],
        }
    }

    /// Build a minimal `/tuio2/bnd` OscMessage (only the 7 required args).
    fn make_minimal_msg(
        session_id: i32,
        x: f32,
        y: f32,
        angle: f32,
        width: f32,
        height: f32,
        area: f32,
    ) -> OscMessage {
        OscMessage {
            addr: "/tuio2/bnd".to_string(),
            args: vec![
                OscType::Int(session_id),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(angle),
                OscType::Float(width),
                OscType::Float(height),
                OscType::Float(area),
            ],
        }
    }

    // ── TryFrom with full args ────────────────────────────────────────────────

    #[test]
    fn full_decodes_session_id() {
        let msg = make_full_msg(13, 0.0, 0.0, 0.0, 0.2, 0.3, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_eq!(bnd.session_id(), 13);
    }

    #[test]
    fn full_decodes_position() {
        let msg = make_full_msg(1, 0.25, 0.75, 0.0, 0.2, 0.3, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.position.x, 0.25);
        assert_relative_eq!(bnd.position.y, 0.75);
    }

    #[test]
    fn full_decodes_angle() {
        let msg = make_full_msg(1, 0.0, 0.0, 2.343, 0.2, 0.3, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.angle, 2.343);
    }

    #[test]
    fn full_decodes_size() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.4, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.size.width, 0.4);
        assert_relative_eq!(bnd.size.height, 0.6);
    }

    #[test]
    fn full_decodes_area() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.24, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.area, 0.24);
    }

    #[test]
    fn full_decodes_velocity() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        let v = bnd.velocity.expect("velocity should be Some");
        assert_relative_eq!(v.x, 1.5);
        assert_relative_eq!(v.y, 2.5);
    }

    #[test]
    fn full_decodes_rotation_speed() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        let rs = bnd.rotation_speed.expect("rotation_speed should be Some");
        assert_relative_eq!(rs, 0.8);
    }

    #[test]
    fn full_decodes_acceleration() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 9.81, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        let a = bnd.acceleration.expect("acceleration should be Some");
        assert_relative_eq!(a, 9.81);
    }

    #[test]
    fn full_decodes_rotation_acceleration() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.1);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        let ra = bnd
            .rotation_acceleration
            .expect("rotation_acceleration should be Some");
        assert_relative_eq!(ra, 1.1);
    }

    // ── TryFrom with minimal args (no optional fields) ────────────────────────

    #[test]
    fn minimal_decodes_session_id() {
        let msg = make_minimal_msg(5, 0.3, 0.8, 1.57, 0.4, 0.6, 0.24);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_eq!(bnd.session_id(), 5);
    }

    #[test]
    fn minimal_decodes_position() {
        let msg = make_minimal_msg(1, 0.3, 0.7, 0.0, 0.4, 0.6, 0.24);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.position.x, 0.3);
        assert_relative_eq!(bnd.position.y, 0.7);
    }

    #[test]
    fn minimal_decodes_angle() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 2.71, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.angle, 2.71);
    }

    #[test]
    fn minimal_decodes_size() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.5, 0.8, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.size.width, 0.5);
        assert_relative_eq!(bnd.size.height, 0.8);
    }

    #[test]
    fn minimal_decodes_area() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.4);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert_relative_eq!(bnd.area, 0.4);
    }

    #[test]
    fn minimal_velocity_is_none() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert!(
            bnd.velocity.is_none(),
            "velocity should be None when not provided"
        );
    }

    #[test]
    fn minimal_rotation_speed_is_none() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert!(bnd.rotation_speed.is_none());
    }

    #[test]
    fn minimal_acceleration_is_none() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert!(bnd.acceleration.is_none());
    }

    #[test]
    fn minimal_rotation_acceleration_is_none() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert!(bnd.rotation_acceleration.is_none());
    }

    // ── Error cases ───────────────────────────────────────────────────────────

    #[test]
    fn empty_message_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/bnd".to_string(),
            args: vec![],
        };
        assert!(BoundsProfile::try_from(&msg).is_err());
    }

    #[test]
    fn too_few_args_returns_error() {
        // 5 args provided — need at least 7.
        let msg = OscMessage {
            addr: "/tuio2/bnd".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Float(0.1),
                OscType::Float(0.2),
                OscType::Float(0.0),
                OscType::Float(0.3),
            ],
        };
        assert!(BoundsProfile::try_from(&msg).is_err());
    }

    #[test]
    fn wrong_type_for_session_id_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/bnd".to_string(),
            args: vec![
                OscType::Float(1.0), // should be Int
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
            ],
        };
        assert!(BoundsProfile::try_from(&msg).is_err());
    }

    // ── From<BoundsProfile> for OscPacket ─────────────────────────────────────

    #[test]
    fn from_full_produces_message_packet() {
        let msg = make_full_msg(1, 0.1, 0.9, 1.57, 0.4, 0.6, 0.24, 0.2, 0.3, 0.5, 0.8, 0.1);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(bnd);
        assert!(matches!(packet, OscPacket::Message(_)));
    }

    #[test]
    fn from_full_address_is_bnd() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.addr, "/tuio2/bnd");
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_has_12_args() {
        let msg = make_full_msg(1, 0.1, 0.9, 1.57, 0.4, 0.6, 0.24, 0.2, 0.3, 0.5, 0.8, 0.1);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args.len(), 12);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_minimal_has_7_args() {
        let msg = make_minimal_msg(1, 0.0, 0.0, 0.0, 0.2, 0.3, 0.06);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args.len(), 7);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_session_id_at_index_0() {
        let msg = make_full_msg(55, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[0], OscType::Int(55));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_position_at_indices_1_2() {
        let msg = make_full_msg(1, 0.3, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[1], OscType::Float(0.3));
            assert_eq!(out.args[2], OscType::Float(0.7));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_angle_at_index_3() {
        let msg = make_full_msg(1, 0.0, 0.0, 1.57, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[3], OscType::Float(1.57));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_size_at_indices_4_5() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.4, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[4], OscType::Float(0.4));
            assert_eq!(out.args[5], OscType::Float(0.6));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_area_at_index_6() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.24, 0.0, 0.0, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[6], OscType::Float(0.24));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_velocity_at_indices_7_8() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[7], OscType::Float(1.5));
            assert_eq!(out.args[8], OscType::Float(2.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_rotation_speed_at_index_9() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[9], OscType::Float(0.9));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_acceleration_at_index_10() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.5, 0.0);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[10], OscType::Float(5.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_full_encodes_rotation_acceleration_at_index_11() {
        let msg = make_full_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.1);
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args[11], OscType::Float(1.1));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    // ── Double round-trip ─────────────────────────────────────────────────────

    #[test]
    fn double_round_trip_full_preserves_all_values() {
        let msg = make_full_msg(7, 0.1, 0.9, 1.57, 0.4, 0.6, 0.24, 0.2, 0.3, 0.5, 0.8, 0.1);
        let bnd1 = BoundsProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(bnd1);
        if let OscPacket::Message(re_encoded) = packet {
            let bnd2 = BoundsProfile::try_from(&re_encoded).unwrap();
            assert_eq!(bnd2.session_id(), 7);
            assert_relative_eq!(bnd2.position.x, 0.1);
            assert_relative_eq!(bnd2.position.y, 0.9);
            assert_relative_eq!(bnd2.angle, 1.57);
            assert_relative_eq!(bnd2.size.width, 0.4);
            assert_relative_eq!(bnd2.size.height, 0.6);
            assert_relative_eq!(bnd2.area, 0.24);
            let v = bnd2.velocity.unwrap();
            assert_relative_eq!(v.x, 0.2);
            assert_relative_eq!(v.y, 0.3);
            assert_relative_eq!(bnd2.rotation_speed.unwrap(), 0.5);
            assert_relative_eq!(bnd2.acceleration.unwrap(), 0.8);
            assert_relative_eq!(bnd2.rotation_acceleration.unwrap(), 0.1);
        } else {
            panic!("expected OscPacket::Message after encoding");
        }
    }

    #[test]
    fn double_round_trip_minimal_preserves_values() {
        let msg = make_minimal_msg(3, 0.2, 0.8, 0.5, 0.3, 0.4, 0.12);
        let bnd1 = BoundsProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(bnd1);
        if let OscPacket::Message(re_encoded) = packet {
            let bnd2 = BoundsProfile::try_from(&re_encoded).unwrap();
            assert_eq!(bnd2.session_id(), 3);
            assert_relative_eq!(bnd2.position.x, 0.2);
            assert_relative_eq!(bnd2.position.y, 0.8);
            assert_relative_eq!(bnd2.angle, 0.5);
            assert_relative_eq!(bnd2.size.width, 0.3);
            assert_relative_eq!(bnd2.size.height, 0.4);
            assert_relative_eq!(bnd2.area, 0.12);
            assert!(bnd2.velocity.is_none());
            assert!(bnd2.rotation_speed.is_none());
            assert!(bnd2.acceleration.is_none());
            assert!(bnd2.rotation_acceleration.is_none());
        } else {
            panic!("expected OscPacket::Message after encoding");
        }
    }

    // ── Partial optional fields ───────────────────────────────────────────────

    #[test]
    fn only_velocity_provided_produces_9_args() {
        // 7 required + 2 velocity = 9 args
        let msg = OscMessage {
            addr: "/tuio2/bnd".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.2),
                OscType::Float(0.3),
                OscType::Float(0.06),
                OscType::Float(1.0), // vx
                OscType::Float(2.0), // vy
            ],
        };
        let bnd = BoundsProfile::try_from(&msg).unwrap();
        assert!(bnd.velocity.is_some());
        assert!(bnd.rotation_speed.is_none());
        assert!(bnd.acceleration.is_none());
        assert!(bnd.rotation_acceleration.is_none());
        if let OscPacket::Message(out) = OscPacket::from(bnd) {
            assert_eq!(out.args.len(), 9);
        }
    }
}
