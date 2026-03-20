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
        self.translation.update(
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
    acceleration: Option<f32>,
    rotation_speed: Option<f32>,
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
            acceleration: if args.remaining() >= 1 {
                Some(args.next_float()?)
            } else {
                None
            },
            rotation_speed: if args.remaining() >= 1 {
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

        args.extend(val.acceleration.into_iter().map(OscType::Float));
        args.extend(val.rotation_speed.into_iter().map(OscType::Float));
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
