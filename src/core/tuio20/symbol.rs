use rosc::{OscMessage, OscPacket, OscType};

use crate::core::{errors::TuioError, osc_utils::ArgCursor, profile::Profile, tuio_time::TuioTime};

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

    pub fn start_time(&self) -> TuioTime {
        self.start_time
    }

    pub fn current_time(&self) -> TuioTime {
        self.current_time
    }

    pub fn session_id(&self) -> i32 {
        self.session_id
    }

    pub fn type_user_id(&self) -> i32 {
        self.type_user_id
    }

    pub fn component_id(&self) -> i32 {
        self.component_id
    }

    pub fn group(&self) -> &str {
        &self.group
    }

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

impl Profile for SymbolProfile {
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
