use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{
    container::Container,
    errors::TuioError,
    math::{Position, Size, Velocity},
    osc_utils::ArgCursor,
    profile::Profile,
    tuio_time::TuioTime,
};

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    container: Container,
    angle: f32,
    size: Size,
    area: f32,
    rotation_speed: Option<f32>,
    rotation_acceleration: Option<f32>,
}

impl Bounds {
    pub(crate) fn new(start_time: &TuioTime, bounds: BoundsProfile) -> Self {
        let container = Container::new(start_time, bounds.session_id, bounds.position);
        Self {
            container,
            angle: bounds.angle,
            size: bounds.size,
            area: bounds.area,
            rotation_speed: bounds.rotation_speed,
            rotation_acceleration: bounds.rotation_acceleration,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, bounds: &BoundsProfile) {
        self.container.update(time, bounds);
        todo!("update bounds fields")
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn area(&self) -> f32 {
        self.area
    }

    pub fn rotation_speed(&self) -> Option<f32> {
        self.rotation_speed
    }

    pub fn rotation_acceleration(&self) -> Option<f32> {
        self.rotation_acceleration
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
            OscType::Float(val.size.x),
            OscType::Float(val.size.y),
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
        "/tuio2/bnd".to_string()
    }
}
