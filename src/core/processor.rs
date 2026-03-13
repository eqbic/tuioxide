use rosc::OscPacket;

pub trait TuioProcessor: Default {
    type Events: Send;
    fn update(&mut self, packet: OscPacket) -> Option<Self::Events>;
}
