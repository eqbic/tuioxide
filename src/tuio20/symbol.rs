use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{ArgCursor, TuioProfile, TuioError, TuioTime};

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
    fn session_id(&self) -> i32 {
        self.session_id
    }

    fn address() -> String {
        "/tuio2/sym".to_string()
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
