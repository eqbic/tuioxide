use std::collections::HashSet;

use rosc::OscMessage;

use crate::{
    core::{ArgCursor, TuioError, TuioTime},
    tuio20::{
        bounds::BoundsProfile, pointer::PointerProfile, symbol::SymbolProfile, token::TokenProfile,
    },
};

#[derive(Debug, Clone, Default)]
struct Components {
    pointers: Vec<PointerProfile>,
    tokens: Vec<TokenProfile>,
    bounds: Vec<BoundsProfile>,
    symbols: Vec<SymbolProfile>,
}

/// Represents a single TUIO **FRM** (/tuio2/frm) message.
///
/// A frame is the unique identifier for an individual TUIO bundle and must
/// appear at the beginning of each bundle. It provides timing, ordering,
/// sensor dimension, and source information for all messages that belong to
/// the same frame.
#[derive(Debug, Clone)]
pub struct Frame {
    frame_id: i32,
    time: TuioTime,
    dimension_x: u16,
    dimension_y: u16,
    source: String,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            frame_id: -1,
            time: TuioTime::default(),
            dimension_x: 0,
            dimension_y: 0,
            source: String::new(),
        }
    }
}

impl Frame {
    /// Returns the frame ID.
    /// This value enables clients to discard late or out-of-order messages by comparing frame IDs.
    /// The value `0` is reserved as a default identifier for out-of-order execution.
    pub fn frame_id(&self) -> i32 {
        self.frame_id
    }

    /// Return the frame timestamp represented as an OSC 64-bit time tag [`TuioTime`].
    pub fn time(&self) -> &TuioTime {
        &self.time
    }

    /// Return the sensor dimensions as a tuple of `(width, height)` in pixels.
    pub fn dimensions(&self) -> (u16, u16) {
        (self.dimension_x, self.dimension_y)
    }

    /// Returns a string uniquely identifying the origin of the TUIO message bundle.
    ///   The format follows the convention:
    ///
    ///   `src_name:src_instance@src_origin`
    ///
    ///   Where:
    ///   - `src_name` is a short identifier of the TUIO server application,
    ///   - `src_instance` differentiates multiple instances,
    ///   - `src_origin` encodes the machine address in hexadecimal form.
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
    components: Components,
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
        &self.components.pointers
    }

    pub(crate) fn tokens(&self) -> &Vec<TokenProfile> {
        &self.components.tokens
    }

    pub(crate) fn bounds(&self) -> &Vec<BoundsProfile> {
        &self.components.bounds
    }

    pub(crate) fn symbols(&self) -> &Vec<SymbolProfile> {
        &self.components.symbols
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
        self.components.pointers.push(pointer);
    }

    pub(crate) fn add_token(&mut self, message: &OscMessage) {
        let token = TokenProfile::try_from(message).unwrap();
        self.components.tokens.push(token);
    }

    pub(crate) fn add_bounds(&mut self, message: &OscMessage) {
        let bound = BoundsProfile::try_from(message).unwrap();
        self.components.bounds.push(bound);
    }

    pub(crate) fn add_symbol(&mut self, message: &OscMessage) {
        let symbol = SymbolProfile::try_from(message).unwrap();
        self.components.symbols.push(symbol);
    }
}
