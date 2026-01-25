use std::collections::{HashMap, HashSet, hash_map};

use rosc::OscMessage;

use crate::{
    common::{
        errors::TuioError,
        osc_utils::{extract_int, extract_string, extract_time},
        tuio_time::TuioTime,
    },
    tuio20::{
        bounds::BoundsProfile, pointer::PointerProfile, symbol::SymbolProfile, token::TokenProfile,
    },
};

pub enum TuioBundleType {}

#[derive(Debug, Clone, Default)]
pub struct Entities {
    pointers: Vec<PointerProfile>,
    tokens: Vec<TokenProfile>,
    bounds: Vec<BoundsProfile>,
    symbols: Vec<SymbolProfile>,
}

#[derive(Debug, Clone, Default)]
pub struct Frame {
    frame_id: i32,
    time: TuioTime,
    dimension_x: u16,
    dimension_y: u16,
    source: String,
}

impl<'a> TryFrom<&'a OscMessage> for Frame {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let frame_id = extract_int(message, 0)?;
        let time = extract_time(message, 1)?;
        let dim_combined = extract_int(message, 2)?;
        let dim_x = ((dim_combined >> 16) & 0xFFFF) as u16;
        let dim_y = (dim_combined & 0xFFFF) as u16;
        let source = extract_string(message, 3)?;
        let frame = Frame {
            frame_id,
            time,
            dimension_x: dim_x,
            dimension_y: dim_y,
            source,
        };
        Ok(frame)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TuioBundle {
    frame: Frame,
    entities: Entities,
    alive: HashSet<i32>,
}

impl TuioBundle {
    pub fn set_alive(&mut self, message: &OscMessage) {
        self.alive = message
            .args
            .iter()
            .filter_map(|e| e.clone().int())
            .collect();
    }

    pub fn set_frame(&mut self, message: &OscMessage) {
        self.frame = Frame::try_from(message).unwrap();
    }

    pub fn add_pointer(&mut self, message: &OscMessage) {
        let pointer = PointerProfile::from_osc_message(message).unwrap();
        self.entities.pointers.push(pointer);
    }

    pub fn add_token(&mut self, message: &OscMessage) {
        let token = TokenProfile::from_osc_message(message).unwrap();
        self.entities.tokens.push(token);
    }

    pub fn add_bounds(&mut self, message: &OscMessage) {
        let bound = BoundsProfile::from_osc_message(message).unwrap();
        self.entities.bounds.push(bound);
    }

    pub fn add_symbol(&mut self, message: &OscMessage) {
        let symbol = SymbolProfile::from_osc_message(message).unwrap();
        self.entities.symbols.push(symbol);
    }
}
