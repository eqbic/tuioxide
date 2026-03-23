use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{ArgCursor, TuioError, TuioProfile, TuioTime};

/// A TUIO 2.0 symbol entity, representing a tagged object with a textual group
/// and data payload, tracked on a touch surface.
///
/// Symbols are identified by a session ID and carry a `group` string (describing
/// the symbol type or namespace) and a `data` string (the actual symbol content,
/// e.g. a barcode value or label). Unlike geometric entities such as [`super::Token`]
/// or [`super::Pointer`], symbols have no spatial position — they carry
/// only identity and textual payload.
///
/// Symbols are delivered via the `/tuio2/sym` OSC address.
#[derive(Debug, Clone)]
pub struct Symbol {
    start_time: TuioTime,
    current_time: TuioTime,
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    group: String,
    data: String,
}

impl Symbol {
    pub(crate) fn new(start_time: &TuioTime, symbol: SymbolProfile) -> Self {
        Self {
            start_time: *start_time,
            current_time: *start_time,
            session_id: symbol.session_id,
            type_user_id: symbol.type_user_id,
            component_id: symbol.component_id,
            group: symbol.group,
            data: symbol.data,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime, symbol: &SymbolProfile) {
        self.current_time = *time;
        self.type_user_id = symbol.type_user_id;
        self.component_id = symbol.component_id;
        self.group = symbol.group.to_owned();
        self.data = symbol.data.to_owned();
    }

    /// Returns the [`TuioTime`] at which this symbol first appeared in the session.
    pub fn start_time(&self) -> TuioTime {
        self.start_time
    }

    /// Returns the [`TuioTime`] of the most recent update to this symbol.
    pub fn current_time(&self) -> TuioTime {
        self.current_time
    }

    /// Returns the session ID uniquely identifying this symbol within the current session.
    pub fn session_id(&self) -> i32 {
        self.session_id
    }

    /// Returns the combined type and user ID associated with this symbol.
    ///
    /// In TUIO 2.0 this field encodes both the type class and the user in a
    /// single integer, following the TUIO 2.0 specification.
    pub fn type_user_id(&self) -> i32 {
        self.type_user_id
    }

    /// Returns the component ID of this symbol.
    ///
    /// The component ID distinguishes multiple simultaneous symbols belonging to
    /// the same type/user combination.
    pub fn component_id(&self) -> i32 {
        self.component_id
    }

    /// Returns the group (namespace or type label) of this symbol.
    ///
    /// For example, this might be `"TUIO"`, `"QRCode"`, or any application-defined
    /// string that categorises the symbol's data format.
    pub fn group(&self) -> &str {
        &self.group
    }

    /// Returns the data payload carried by this symbol.
    ///
    /// The meaning of this string depends on the `group`. For example, for a
    /// barcode group it might be the raw barcode text; for a custom group it
    /// could be any application-defined value.
    pub fn data(&self) -> &str {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SymbolProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    group: String,
    data: String,
}

impl TuioProfile for SymbolProfile {
    fn address() -> String {
        "/tuio2/sym".to_string()
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }
}

impl<'a> TryFrom<&'a OscMessage> for SymbolProfile {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 0);
        Ok(SymbolProfile {
            session_id: args.next_int()?,
            type_user_id: args.next_int()?,
            component_id: args.next_int()?,
            group: args.next_string()?,
            data: args.next_string()?,
        })
    }
}

impl From<SymbolProfile> for OscPacket {
    fn from(val: SymbolProfile) -> Self {
        OscPacket::Message(OscMessage {
            addr: SymbolProfile::address(),
            args: vec![
                OscType::Int(val.session_id),
                OscType::Int(val.type_user_id),
                OscType::Int(val.component_id),
                OscType::String(val.group),
                OscType::String(val.data),
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use rosc::{OscMessage, OscPacket, OscType};

    use crate::core::TuioProfile;

    use super::SymbolProfile;

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn make_msg(
        session_id: i32,
        type_user_id: i32,
        component_id: i32,
        group: &str,
        data: &str,
    ) -> OscMessage {
        OscMessage {
            addr: "/tuio2/sym".to_string(),
            args: vec![
                OscType::Int(session_id),
                OscType::Int(type_user_id),
                OscType::Int(component_id),
                OscType::String(group.to_string()),
                OscType::String(data.to_string()),
            ],
        }
    }

    // ── TryFrom<&OscMessage> ─────────────────────────────────────────────────

    #[test]
    fn decodes_session_id() {
        let msg = make_msg(42, 0, 0, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.session_id(), 42);
    }

    #[test]
    fn decodes_type_user_id() {
        let msg = make_msg(1, 99, 0, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.type_user_id, 99);
    }

    #[test]
    fn decodes_component_id() {
        let msg = make_msg(1, 0, 7, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.component_id, 7);
    }

    #[test]
    fn decodes_group() {
        let msg = make_msg(1, 0, 0, "TUIO", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.group, "TUIO");
    }

    #[test]
    fn decodes_data() {
        let msg = make_msg(1, 0, 0, "G", "hello_world");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.data, "hello_world");
    }

    #[test]
    fn decodes_empty_group_string() {
        let msg = make_msg(1, 0, 0, "", "some_data");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.group, "");
    }

    #[test]
    fn decodes_empty_data_string() {
        let msg = make_msg(1, 0, 0, "some_group", "");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        assert_eq!(sym.data, "");
    }

    // ── Error cases ───────────────────────────────────────────────────────────

    #[test]
    fn empty_message_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/sym".to_string(),
            args: vec![],
        };
        assert!(SymbolProfile::try_from(&msg).is_err());
    }

    #[test]
    fn too_few_args_returns_error() {
        // Only 3 args provided — need 5.
        let msg = OscMessage {
            addr: "/tuio2/sym".to_string(),
            args: vec![OscType::Int(1), OscType::Int(0), OscType::Int(0)],
        };
        assert!(SymbolProfile::try_from(&msg).is_err());
    }

    #[test]
    fn wrong_type_for_session_id_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/sym".to_string(),
            args: vec![
                OscType::Float(1.0), // should be Int
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("G".to_string()),
                OscType::String("D".to_string()),
            ],
        };
        assert!(SymbolProfile::try_from(&msg).is_err());
    }

    #[test]
    fn wrong_type_for_group_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/sym".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Int(0),
                OscType::Int(0),
                OscType::Int(99), // should be String
                OscType::String("D".to_string()),
            ],
        };
        assert!(SymbolProfile::try_from(&msg).is_err());
    }

    #[test]
    fn wrong_type_for_data_returns_error() {
        let msg = OscMessage {
            addr: "/tuio2/sym".to_string(),
            args: vec![
                OscType::Int(1),
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("G".to_string()),
                OscType::Float(PI), // should be String
            ],
        };
        assert!(SymbolProfile::try_from(&msg).is_err());
    }

    // ── From<SymbolProfile> for OscPacket ────────────────────────────────────

    #[test]
    fn from_produces_message_packet() {
        let msg = make_msg(1, 2, 3, "mygroup", "mydata");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(sym);
        assert!(matches!(packet, OscPacket::Message(_)));
    }

    #[test]
    fn from_address_is_sym() {
        let msg = make_msg(1, 0, 0, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(out.addr, "/tuio2/sym");
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_has_5_args() {
        let msg = make_msg(1, 2, 3, "mygroup", "mydata");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(out.args.len(), 5);
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_session_id_at_index_0() {
        let msg = make_msg(77, 0, 0, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(out.args[0], OscType::Int(77));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_type_user_id_at_index_1() {
        let msg = make_msg(1, 55, 0, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(out.args[1], OscType::Int(55));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_component_id_at_index_2() {
        let msg = make_msg(1, 0, 33, "G", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(out.args[2], OscType::Int(33));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_group_at_index_3() {
        let msg = make_msg(1, 0, 0, "QRCode", "D");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(out.args[3], OscType::String("QRCode".to_string()));
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    #[test]
    fn from_encodes_data_at_index_4() {
        let msg = make_msg(1, 0, 0, "G", "barcode_value_123");
        let sym = SymbolProfile::try_from(&msg).unwrap();
        if let OscPacket::Message(out) = OscPacket::from(sym) {
            assert_eq!(
                out.args[4],
                OscType::String("barcode_value_123".to_string())
            );
        } else {
            panic!("expected OscPacket::Message");
        }
    }

    // ── Round-trip: decode → re-encode → decode ───────────────────────────────

    #[test]
    fn round_trip_preserves_all_fields() {
        let msg = make_msg(10, 20, 30, "namespace", "payload");
        let sym1 = SymbolProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(sym1);
        if let OscPacket::Message(re_encoded) = packet {
            let sym2 = SymbolProfile::try_from(&re_encoded).unwrap();
            assert_eq!(sym2.session_id(), 10);
            assert_eq!(sym2.type_user_id, 20);
            assert_eq!(sym2.component_id, 30);
            assert_eq!(sym2.group, "namespace");
            assert_eq!(sym2.data, "payload");
        } else {
            panic!("expected OscPacket::Message after encoding");
        }
    }

    #[test]
    fn round_trip_with_special_characters_in_strings() {
        let msg = make_msg(1, 0, 0, "group/sub", "data:value=42&foo=bar");
        let sym1 = SymbolProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(sym1);
        if let OscPacket::Message(re_encoded) = packet {
            let sym2 = SymbolProfile::try_from(&re_encoded).unwrap();
            assert_eq!(sym2.group, "group/sub");
            assert_eq!(sym2.data, "data:value=42&foo=bar");
        } else {
            panic!("expected OscPacket::Message after encoding");
        }
    }

    #[test]
    fn round_trip_with_negative_ids() {
        // session IDs and type/component IDs can technically be any i32
        let msg = make_msg(-1, -2, -3, "G", "D");
        let sym1 = SymbolProfile::try_from(&msg).unwrap();
        let packet = OscPacket::from(sym1);
        if let OscPacket::Message(re_encoded) = packet {
            let sym2 = SymbolProfile::try_from(&re_encoded).unwrap();
            assert_eq!(sym2.session_id(), -1);
            assert_eq!(sym2.type_user_id, -2);
            assert_eq!(sym2.component_id, -3);
        } else {
            panic!("expected OscPacket::Message after encoding");
        }
    }

    // ── address() ─────────────────────────────────────────────────────────────

    #[test]
    fn address_is_tuio2_sym() {
        use crate::core::TuioProfile;
        assert_eq!(SymbolProfile::address(), "/tuio2/sym");
    }
}
