use rosc::OscPacket;

pub trait Profile: Into<OscPacket> {
    fn session_id(&self) -> i32;
    fn address() -> String;
}
