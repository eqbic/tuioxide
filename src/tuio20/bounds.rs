use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Profile, Rotation, Size, Translation, TuioError, TuioTime,
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
        self.translation.update(
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

impl Profile for BoundsProfile {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio2/bnd".to_string()
    }
}
