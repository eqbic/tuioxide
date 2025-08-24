use rosc::{OscMessage, OscPacket};

pub trait Profile<'a>: TryFrom<&'a OscMessage> + Into<OscPacket> {
    fn session_id(&self) -> i32;
    fn address() -> String;
}
