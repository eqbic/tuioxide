use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Velocity},
    osc_utils::ArgCursor,
    profile::Profile,
    rotation::Rotation,
    translation::Translation,
    tuio_time::TuioTime,
};

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
        todo!("update token fields")
    }

    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
    }

    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    pub fn session_id(&self) -> i32 {
        self.container.session_id
    }

    pub fn type_user_id(&self) -> i32 {
        self.type_user_id
    }

    pub fn component_id(&self) -> i32 {
        self.component_id
    }

    pub fn position(&self) -> Position {
        self.translation.position
    }

    pub fn velocity(&self) -> Velocity {
        self.translation.velocity
    }

    pub fn speed(&self) -> f32 {
        self.translation.velocity.speed()
    }

    pub fn angle(&self) -> f32 {
        self.rotation.angle
    }

    pub fn rotation_speed(&self) -> f32 {
        self.rotation.speed
    }

    pub fn rotation_acceleration(&self) -> f32 {
        self.rotation.acceleration
    }
}

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

impl Profile for TokenProfile {
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn position(&self) -> Position {
        self.position
    }

    fn velocity(&self) -> Velocity {
        self.velocity.unwrap_or_default()
    }

    fn acceleration(&self) -> f32 {
        self.acceleration.unwrap_or_default()
    }

    fn address() -> String {
        "/tuio2/tok".to_string()
    }
}
