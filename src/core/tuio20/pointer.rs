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
        let container = Container::new(start_time, pointer.session_id, pointer.position);
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
        self.container.update(time, pointer);
        self.translation.update(
            pointer.position,
            pointer.velocity.unwrap_or_default(),
            pointer.acceleration.unwrap_or_default(),
        );
        self.rotation.update(pointer.angle, 0.0, 0.0);
        todo!("update pointer fields")
    }

    pub fn current_time(&self) -> TuioTime {
        self.container.current_time
    }

    pub fn start_time(&self) -> TuioTime {
        self.container.start_time
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

    pub fn acceleration(&self) -> f32 {
        self.translation.acceleration
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

    pub fn shear(&self) -> f32 {
        self.shear
    }

    pub fn pressure(&self) -> f32 {
        self.pressure
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn pressure_speed(&self) -> Option<f32> {
        self.pressure_speed
    }

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

impl Profile for PointerProfile {
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
        "/tuio2/ptr".to_string()
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
