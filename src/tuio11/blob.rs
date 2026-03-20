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
        let velocity = Velocity::new(args.next_float()?, args.next_float()?);
        let acceleration = args.next_float()?;
        let angle = args.next_float()?;
        let size = Size::new(args.next_float()?, args.next_float()?);
        let area = args.next_float()?;
        let rotation_speed = args.next_float()?;
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
    pub(crate) fn new(
        start_time: &TuioTime,
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
        let container = Container::new(start_time, session_id);
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
