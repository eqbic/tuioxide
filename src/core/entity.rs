use rosc::OscPacket;

/// Trait representing a TUIO entity — a typed, addressable OSC entity
/// (e.g. a cursor, object, blob, pointer, token, symbol, or bounds).
///
/// Each entity type maps to a specific OSC address (e.g. `/tuio/2Dcur`)
/// and can be converted into an [`OscPacket`] for transmission.
pub trait TuioEntity: Into<OscPacket> + Clone {
    /// Returns the session ID of this entity instance.
    ///
    /// Session IDs are assigned by the TUIO source and uniquely identify
    /// an active entity within a session.
    fn session_id(&self) -> i32;

    /// Returns the OSC address string for this entity type.
    ///
    /// For example, the 2D cursor profile returns `"/tuio/2Dcur"` and
    /// the TUIO 2.0 pointer profile returns `"/tuio2/ptr"`.
    fn address() -> String;
}
