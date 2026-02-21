use rosc::OscPacket;

use crate::core::math::{Position, Velocity};

pub trait Profile: Into<OscPacket> {
    fn session_id(&self) -> i32;
    fn position(&self) -> Position;
    fn velocity(&self) -> Velocity;
    fn acceleration(&self) -> f32;
    fn address() -> String;
}
