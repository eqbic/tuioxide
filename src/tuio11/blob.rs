use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Rotation, Size, Translation, TuioError, TuioProfile, TuioTime,
    Velocity,
};

/// A TUIO 1.1 blob entity, corresponding to the `/tuio/2Dblb` profile.
///
/// A blob represents an amorphous contact region on a 2D surface. In addition to
/// the position, velocity and acceleration properties shared with cursors and
/// objects, a blob also carries orientation (`angle`), bounding `size`, rotational
/// motion, and a scalar `area` that describes how much surface it covers.
///
/// Blobs are produced by the TUIO 1.1 client processor and delivered via
/// [`BlobEvent`](crate::tuio11::event::BlobEvent).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Blob {
    container: Container,
    translation: Translation,
    rotation: Rotation,
    size: Size,
    area: f32,
}

impl TuioProfile for Blob {
    fn address() -> String {
        "/tuio/2Dblb".into()
    }

    fn session_id(&self) -> i32 {
        self.session_id()
    }
}

impl From<Blob> for OscPacket {
    fn from(blob: Blob) -> Self {
        OscPacket::Message(OscMessage {
            addr: Blob::address(),
            args: vec![
                OscType::String("set".into()),
                OscType::Int(blob.session_id()),
                OscType::Float(blob.position().x),
                OscType::Float(blob.position().y),
                OscType::Float(blob.angle()),
                OscType::Float(blob.size().width),
                OscType::Float(blob.size().height),
                OscType::Float(blob.area()),
                OscType::Float(blob.velocity().x),
                OscType::Float(blob.velocity().y),
                OscType::Float(blob.rotation_speed()),
                OscType::Float(blob.acceleration()),
                OscType::Float(blob.rotation_acceleration()),
            ],
        })
    }
}

impl<'a> TryFrom<&'a OscMessage> for Blob {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 1);
        let session_id = args.next_int()?;
        let position = Position::new(args.next_float()?, args.next_float()?);
        let angle = args.next_float()?;
        let size = Size::new(args.next_float()?, args.next_float()?);
        let area = args.next_float()?;
        let velocity = Velocity::new(args.next_float()?, args.next_float()?);
        let rotation_speed = args.next_float()?;
        let acceleration = args.next_float()?;
        let rotation_acceleration = args.next_float()?;

        let container = Container::new(&TuioTime::from_system_time().unwrap(), session_id);
        let translation = Translation::new(position, velocity, acceleration);
        let rotation = Rotation::new(angle, rotation_speed, rotation_acceleration);
        Ok(Blob {
            container,
            translation,
            rotation,
            size,
            area,
        })
    }
}

impl Blob {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_id: i32,
        position: Position,
        velocity: Velocity,
        acceleration: f32,
        angle: f32,
        rotation_speed: f32,
        rotation_acceleration: f32,
        size: Size,
        area: f32,
    ) -> Self {
        let container = Container::new(&TuioTime::from_system_time().unwrap(), session_id);
        let translation = Translation::new(position, velocity, acceleration);
        let rotation = Rotation::new(angle, rotation_speed, rotation_acceleration);
        Self {
            container,
            translation,
            rotation,
            size,
            area,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, blob: &Blob) {
        self.container.update(time);
        self.translation = blob.translation;
        self.rotation = blob.rotation;
        self.size = blob.size;
        self.area = blob.area;
    }

    /// Returns the [`TuioTime`] at which this blob was last updated.
    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    /// Returns the [`TuioTime`] at which this blob first appeared.
    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    /// Returns the unique session ID assigned to this blob by the TUIO source.
    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    /// Returns the current normalized position of this blob on the surface.
    ///
    /// Coordinates are in the range `[0.0, 1.0]` for both axes.
    pub fn position(&self) -> Position {
        self.translation.position
    }

    /// Returns the current 2D velocity vector of this blob.
    ///
    /// Each component represents the rate of change of the corresponding
    /// position component per frame.
    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    /// Returns the scalar translational acceleration of this blob.
    ///
    /// Positive values indicate speeding up; negative values indicate slowing down.
    pub fn acceleration(&self) -> f32 {
        self.translation.acceleration
    }

    /// Returns the current orientation angle of this blob, in radians.
    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    /// Returns the current rotational speed of this blob, in radians per frame.
    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    /// Returns the rotational acceleration of this blob, in radians per frame squared.
    ///
    /// Positive values indicate increasing rotational speed; negative values indicate
    /// decreasing rotational speed.
    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }

    /// Returns the bounding size of this blob in normalized surface coordinates.
    ///
    /// The [`Size`] contains `width` and `height` components, both in the range
    /// `[0.0, 1.0]`.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns the surface area covered by this blob, in normalized units.
    pub fn area(&self) -> f32 {
        self.area
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;
    use rosc::{OscMessage, OscPacket, OscType};

    use crate::core::{Position, Size, Velocity};

    use super::Blob;

    // ── Helpers ──────────────────────────────────────────────────────────────

    /// Build a well-formed `/tuio/2Dblb` "set" OscMessage with 13 args.
    ///
    /// Decode order (per TryFrom):
    ///   [0] "set", [1] session_id, [2] x, [3] y, [4] vx, [5] vy,
    ///   [6] accel, [7] angle, [8] width, [9] height, [10] area,
    ///   [11] rot_speed, [12] rot_accel
    #[allow(clippy::too_many_arguments)]
    fn make_set_msg(
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
            addr: "/tuio/2Dblb".to_string(),
            args: vec![
                OscType::String("set".into()),
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

    fn default_msg() -> OscMessage {
        make_set_msg(3, 0.1, 0.9, 0.2, 0.3, 0.5, 1.57, 0.4, 0.6, 0.24, 0.8, 0.1)
    }

    // ── TryFrom<&OscMessage> ─────────────────────────────────────────────────

    #[test]
    fn try_from_decodes_session_id() {
        let msg = make_set_msg(42, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_eq!(blob.session_id(), 42);
    }

    #[test]
    fn try_from_decodes_position() {
        let msg = make_set_msg(1, 0.25, 0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.position().x, 0.25);
        assert_relative_eq!(blob.position().y, 0.75);
    }

    #[test]
    fn try_from_decodes_velocity() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.velocity().x, 1.5);
        assert_relative_eq!(blob.velocity().y, 2.5);
    }

    #[test]
    fn try_from_decodes_acceleration() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.4, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.acceleration(), 3.4);
    }

    #[test]
    fn try_from_decodes_angle() {
        let msg = make_set_msg(1, 0.0, 0.0, 1.57, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.angle(), 1.57);
    }

    #[test]
    fn try_from_decodes_size() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.4, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.size().width, 0.4);
        assert_relative_eq!(blob.size().height, 0.6);
    }

    #[test]
    fn try_from_decodes_area() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.24, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.area(), 0.24);
    }

    #[test]
    fn try_from_decodes_rotation_speed() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.rotation_speed(), 0.7)
    }

    #[test]
    fn try_from_decodes_rotation_acceleration() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.1);
        let blob = Blob::try_from(&msg).unwrap();
        assert_relative_eq!(blob.rotation_acceleration(), 1.1);
    }

    #[test]
    fn try_from_missing_args_returns_error() {
        // Only 5 args instead of 13.
        let msg = OscMessage {
            addr: "/tuio/2Dblb".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Int(1),
                OscType::Float(0.1),
                OscType::Float(0.2),
                OscType::Float(0.3),
            ],
        };
        assert!(Blob::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_wrong_type_for_session_id_returns_error() {
        let msg = OscMessage {
            addr: "/tuio/2Dblb".to_string(),
            args: vec![
                OscType::String("set".to_string()),
                OscType::Float(1.0), // should be Int
                OscType::Float(0.0),
                OscType::Float(0.0),
                OscType::Float(0.0),
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
        assert!(Blob::try_from(&msg).is_err());
    }

    #[test]
    fn try_from_empty_message_returns_error() {
        let msg = OscMessage {
            addr: "/tuio/2Dblb".to_string(),
            args: vec![],
        };
        assert!(Blob::try_from(&msg).is_err());
    }

    // ── From<Blob> for OscPacket ──────────────────────────────────────────────
    //
    // NOTE: The encoding order differs from the decoding order:
    //   Encode: [1]=session_id, [2]=x, [3]=y, [4]=angle, [5]=width, [6]=height,
    //           [7]=area, [8]=vx, [9]=vy, [10]=rot_speed, [11]=accel, [12]=rot_accel

    #[test]
    fn from_produces_message_packet() {
        let blob = Blob::try_from(&default_msg()).unwrap();
        let packet = OscPacket::from(blob);
        assert!(matches!(packet, OscPacket::Message(_)));
    }

    #[test]
    fn from_address_is_2dblb() {
        let blob = Blob::try_from(&default_msg()).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.addr, "/tuio/2Dblb");
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_first_arg_is_set() {
        let blob = Blob::try_from(&default_msg()).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[0], OscType::String("set".to_string()));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_has_13_args() {
        let blob = Blob::try_from(&default_msg()).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args.len(), 13);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_session_id_at_index_1() {
        let msg = make_set_msg(7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[1], OscType::Int(7));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_position_at_indices_2_3() {
        let msg = make_set_msg(1, 0.3, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[2], OscType::Float(0.3));
            assert_eq!(out.args[3], OscType::Float(0.7));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_angle_at_index_4() {
        let msg = make_set_msg(1, 0.0, 0.0, 1.57, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[4], OscType::Float(1.57));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_size_at_indices_5_6() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.4, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[5], OscType::Float(0.4));
            assert_eq!(out.args[6], OscType::Float(0.6));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_area_at_index_7() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.24, 0.0, 0.0, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[7], OscType::Float(0.24));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_velocity_at_indices_8_9() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 2.5, 0.0, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[8], OscType::Float(1.5));
            assert_eq!(out.args[9], OscType::Float(2.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_rotation_speed_at_index_10() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[10], OscType::Float(0.9));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_acceleration_at_index_11() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.5, 0.0);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[11], OscType::Float(5.5));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_rotation_acceleration_at_index_12() {
        let msg = make_set_msg(1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.1);
        let blob = Blob::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(blob) {
            assert_eq!(out.args[12], OscType::Float(1.1));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    // ── Double round-trip: decode → re-encode → decode → check values ─────────

    #[test]
    fn double_round_trip_preserves_values() {
        // Decode a message, encode it to a packet, decode that packet, and
        // verify all field values survived both transformations.
        let msg = make_set_msg(5, 0.1, 0.9, 0.2, 0.3, 0.5, 1.57, 0.4, 0.6, 0.24, 0.8, 0.1);
        let blob1 = Blob::try_from(&msg).unwrap();
        let packet = OscPacket::from(blob1);
        if let OscPacket::Message(re_encoded) = packet {
            let blob2 = Blob::try_from(&re_encoded).unwrap();
            assert_eq!(blob2.session_id(), 5);
            assert_relative_eq!(blob2.position().x, 0.1);
            assert_relative_eq!(blob2.position().y, 0.9);
            assert_relative_eq!(blob2.angle(), 0.2);
            assert_relative_eq!(blob2.size().width, 0.3);
            assert_relative_eq!(blob2.size().height, 0.5);
            assert_relative_eq!(blob2.area(), 1.57);
            assert_relative_eq!(blob2.velocity().x, 0.4);
            assert_relative_eq!(blob2.velocity().y, 0.6);
            assert_relative_eq!(blob2.rotation_speed(), 0.24);
            assert_relative_eq!(blob2.acceleration(), 0.8);
            assert_relative_eq!(blob2.rotation_acceleration(), 0.1);
        } else {
            panic!("expected OscPacket::Message after encoding");
        }
    }

    // ── Blob::new ─────────────────────────────────────────────────────────────

    #[test]
    fn new_stores_all_fields() {
        let blob = Blob::new(
            9,
            Position::new(0.1, 0.2),
            Velocity::new(0.3, 0.4),
            1.5,
            0.8,
            0.2,
            0.1,
            Size::new(0.5, 0.6),
            0.3,
        );
        assert_eq!(blob.session_id(), 9);
        assert_relative_eq!(blob.position().x, 0.1);
        assert_relative_eq!(blob.position().y, 0.2);
        assert_relative_eq!(blob.velocity().x, 0.3);
        assert_relative_eq!(blob.velocity().y, 0.4);
        assert_relative_eq!(blob.acceleration(), 1.5);
        assert_relative_eq!(blob.angle(), 0.8);
        assert_relative_eq!(blob.rotation_speed(), 0.2);
        assert_relative_eq!(blob.rotation_acceleration(), 0.1);
        assert_relative_eq!(blob.size().width, 0.5);
        assert_relative_eq!(blob.size().height, 0.6);
        assert_relative_eq!(blob.area(), 0.3);
    }

    #[test]
    fn new_start_time_equals_current_time_initially() {
        let blob = Blob::new(
            1,
            Position::new(0.0, 0.0),
            Velocity::new(0.0, 0.0),
            0.0,
            0.0,
            0.0,
            0.0,
            Size::new(0.0, 0.0),
            0.0,
        );
        assert_eq!(blob.start_time(), blob.current_time());
    }
}
