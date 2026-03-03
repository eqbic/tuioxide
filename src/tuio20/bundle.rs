use std::collections::HashSet;

use rosc::OscMessage;

use crate::{
    core::{ArgCursor, TuioError, TuioTime},
    tuio20::{
        bounds::BoundsProfile, pointer::PointerProfile, symbol::SymbolProfile, token::TokenProfile,
    },
};

#[derive(Debug, Clone, Default)]
struct Entities {
    pointers: Vec<PointerProfile>,
    tokens: Vec<TokenProfile>,
    bounds: Vec<BoundsProfile>,
    symbols: Vec<SymbolProfile>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Frame {
    frame_id: i32,
    time: TuioTime,
    dimension_x: u16,
    dimension_y: u16,
    source: String,
}

impl Frame {
    pub fn frame_id(&self) -> i32 {
        self.frame_id
    }

    pub fn time(&self) -> &TuioTime {
        &self.time
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.dimension_x, self.dimension_y)
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

impl<'a> TryFrom<&'a OscMessage> for Frame {
    type Error = TuioError;

    fn try_from(message: &'a OscMessage) -> Result<Self, Self::Error> {
        let mut args = ArgCursor::new(message, 0);
        let frame_id = args.next_int()?;
        let time = args.next_time()?;
        let dim_combined = args.next_int()?;
        let dim_x = ((dim_combined >> 16) & 0xFFFF) as u16;
        let dim_y = (dim_combined & 0xFFFF) as u16;
        let source = args.next_string()?;
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
pub(crate) struct TuioBundle {
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

    pub(crate) fn pointers(&self) -> &Vec<PointerProfile> {
        &self.entities.pointers
    }

    pub(crate) fn tokens(&self) -> &Vec<TokenProfile> {
        &self.entities.tokens
    }

    pub(crate) fn bounds(&self) -> &Vec<BoundsProfile> {
        &self.entities.bounds
    }

    pub(crate) fn symbols(&self) -> &Vec<SymbolProfile> {
        &self.entities.symbols
    }

    pub(crate) fn frame(&self) -> &Frame {
        &self.frame
    }

    pub(crate) fn alive(&self) -> &HashSet<i32> {
        &self.alive
    }

    pub(crate) fn set_frame(&mut self, message: &OscMessage) {
        self.frame = Frame::try_from(message).unwrap();
    }

    pub(crate) fn add_pointer(&mut self, message: &OscMessage) {
        let pointer = PointerProfile::try_from(message).unwrap();
        self.entities.pointers.push(pointer);
    }

    pub(crate) fn add_token(&mut self, message: &OscMessage) {
        let token = TokenProfile::try_from(message).unwrap();
        self.entities.tokens.push(token);
    }

    pub(crate) fn add_bounds(&mut self, message: &OscMessage) {
        let bound = BoundsProfile::try_from(message).unwrap();
        self.entities.bounds.push(bound);
    }

    pub(crate) fn add_symbol(&mut self, message: &OscMessage) {
        let symbol = SymbolProfile::try_from(message).unwrap();
        self.entities.symbols.push(symbol);
    }
}
