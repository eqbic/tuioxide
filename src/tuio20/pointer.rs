use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    ArgCursor, Container, Position, Rotation, Translation, TuioError, TuioProfile, TuioTime,
    Velocity,
};

/// A TUIO 2.0 pointer entity, corresponding to the `/tuio2/ptr` profile.
///
/// A pointer represents a single contact point from an input device such as a
/// finger, stylus, or mouse. In addition to position and motion data it carries
/// pressure, radius, and shear information that richer input devices can supply.
///
/// Optional fields (`pressure_speed`, `pressure_acceleration`) are `None` when
/// the source does not include them in the OSC message.
#[derive(Debug, Clone, Copy)]
pub struct Pointer {
    container: Container,
    type_user_id: i32,
    component_id: i32,
    translation: Translation,
    rotation: Rotation,
    shear: f32,
    radius: f32,
    pressure: f32,
    pressure_speed: Option<f32>,
    pressure_acceleration: Option<f32>,
}

impl Pointer {
    pub(crate) fn new(start_time: &TuioTime, pointer: PointerProfile) -> Self {
        let container = Container::new(start_time, pointer.session_id);
        let translation = Translation::new(
            pointer.position,
            pointer.velocity.unwrap_or_default(),
            pointer.acceleration.unwrap_or_default(),
        );
        let rotation = Rotation::new(pointer.angle, 0.0, 0.0);
        Self {
            container,
            type_user_id: pointer.type_user_id,
            component_id: pointer.component_id,
            translation,
            rotation,
            shear: pointer.shear,
            radius: pointer.radius,
            pressure: pointer.pressure,
            pressure_speed: pointer.pressure_speed,
            pressure_acceleration: pointer.pressure_acceleration,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, pointer: &PointerProfile) {
        self.container.update(time);
        self.translation.update(
            pointer.position,
            pointer.velocity.unwrap_or_default(),
            pointer.acceleration.unwrap_or_default(),
        );
        self.rotation.update(pointer.angle, 0.0, 0.0);
        self.shear = pointer.shear;
        self.radius = pointer.radius;
        self.pressure = pointer.pressure;
        self.pressure_speed = pointer.pressure_speed;
        self.pressure_acceleration = pointer.pressure_acceleration;
    }

    /// Returns the [`TuioTime`] at which this pointer was first observed.
    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    /// Returns the [`TuioTime`] of the most recent update for this pointer.
    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    /// Returns the session ID that uniquely identifies this pointer within the
    /// current TUIO session.
    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    /// Returns the combined type/user ID field.
    ///
    /// The upper 16 bits encode the type ID and the lower 16 bits encode the
    /// user ID, as defined by the TUIO 2.0 specification.
    pub fn type_user_id(&self) -> i32 {
        self.type_user_id
    }

    /// Returns the component ID of this pointer.
    ///
    /// The component ID distinguishes individual contact points belonging to the
    /// same type/user combination (e.g. different fingers of the same hand).
    pub fn component_id(&self) -> i32 {
        self.component_id
    }

    /// Returns the current normalized position of this pointer in the range
    /// `[0.0, 1.0]` for both axes.
    pub fn position(&self) -> Position {
        self.translation.position
    }

    /// Returns the current 2D velocity vector of this pointer.
    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    /// Returns the scalar speed of this pointer (the Euclidean magnitude of its
    /// velocity vector).
    pub fn speed(&self) -> f32 {
        self.translation.velocity.speed()
    }

    /// Returns the current translational acceleration of this pointer.
    pub fn acceleration(&self) -> f32 {
        self.translation.acceleration
    }

    /// Returns the current orientation angle of this pointer, in radians.
    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    /// Returns the rotational speed of this pointer, in radians per frame.
    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    /// Returns the rotational acceleration of this pointer, in radians per frame².
    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }

    /// Returns the shear value of this pointer.
    ///
    /// Shear represents the lateral tilt of a stylus or similar device, as
    /// defined by the TUIO 2.0 specification.
    pub fn shear(&self) -> f32 {
        self.shear
    }

    /// Returns the contact pressure of this pointer, normalized to `[0.0, 1.0]`.
    pub fn pressure(&self) -> f32 {
        self.pressure
    }

    /// Returns the contact radius of this pointer in normalized surface units.
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Returns the rate of change of contact pressure, if provided by the source.
    ///
    /// Returns `None` when the TUIO source does not transmit this optional field.
    pub fn pressure_speed(&self) -> Option<f32> {
        self.pressure_speed
    }

    /// Returns the acceleration of contact pressure, if provided by the source.
    ///
    /// Returns `None` when the TUIO source does not transmit this optional field.
    pub fn pressure_acceleration(&self) -> Option<f32> {
        self.pressure_acceleration
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PointerProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    position: Position,
    angle: f32,
    shear: f32,
    radius: f32,
    pressure: f32,
    velocity: Option<Velocity>,
    pressure_speed: Option<f32>,
    acceleration: Option<f32>,
    pressure_acceleration: Option<f32>,
}

impl<'a> TryFrom<&'a OscMessage> for PointerProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 0);
        Ok(PointerProfile {
            session_id: args.next_int()?,
            type_user_id: args.next_int()?,
            component_id: args.next_int()?,
            position: Position::new(args.next_float()?, args.next_float()?),
            angle: args.next_float()?,
            shear: args.next_float()?,
            radius: args.next_float()?,
            pressure: args.next_float()?,
            velocity: if args.remaining() >= 2 {
                Some(Velocity::new(args.next_float()?, args.next_float()?))
            } else {
                None
            },
            pressure_speed: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
            acceleration: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
            pressure_acceleration: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
        })
    }
}

impl From<PointerProfile> for OscPacket {
    fn from(val: PointerProfile) -> Self {
        let mut args = vec![
            OscType::Int(val.session_id),
            OscType::Int(val.type_user_id),
            OscType::Int(val.component_id),
            OscType::Float(val.position.x),
            OscType::Float(val.position.y),
            OscType::Float(val.angle),
            OscType::Float(val.shear),
            OscType::Float(val.radius),
            OscType::Float(val.pressure),
        ];

        if let Some(velocity) = val.velocity {
            args.extend([OscType::Float(velocity.x), OscType::Float(velocity.y)]);
        }

        args.extend(val.pressure_speed.into_iter().map(OscType::Float));
        args.extend(val.acceleration.into_iter().map(OscType::Float));
        args.extend(val.pressure_acceleration.into_iter().map(OscType::Float));

        OscPacket::Message(OscMessage {
            addr: PointerProfile::address(),
            args,
        })
    }
}

impl TuioProfile for PointerProfile {
    fn address() -> String {
        "/tuio2/ptr".to_string()
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }
}

#[cfg(test)]
mod tests {
    use rosc::{OscMessage, OscType};

    use super::*;

    #[test]
    fn test_decode() {
        let msg = OscMessage {
            addr: "/tuio2/ptr".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Int(2),
                OscType::Int(3),
                OscType::Float(0.5),
                OscType::Float(0.7),
                OscType::Float(1.2),
                OscType::Float(0.5),
                OscType::Float(0.7),
                OscType::Float(1.2),
                OscType::Float(0.5),
                OscType::Float(0.7),
            ],
        };

        let pointer = match PointerProfile::try_from(&msg) {
            Ok(pointer) => pointer,
            Err(error) => panic!("{error:?}"),
        };

        assert_eq!(pointer.component_id, 3);

        assert_eq!(OscPacket::from(pointer), OscPacket::Message(msg));
    }
}
