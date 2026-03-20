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
    fn session_id(&self) -> i32 {
        self.session_id()
    }

    fn address() -> String {
        "/tuio/2Dobj".into()
    }
}

impl Object {
    pub(crate) fn new(
        start_time: &TuioTime,
        session_id: i32,
        class_id: i32,
        position: Position,
        velocity: Velocity,
        acceleration: f32,
        angle: f32,
        rotation_speed: f32,
        rotation_acceleration: f32,
    ) -> Self {
        let container = Container::new(start_time, session_id);
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
