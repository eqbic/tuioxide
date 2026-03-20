use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
};

use rosc::OscPacket;

use crate::{
    core::{TuioProfile, TuioTime, processor::TuioProcessor, retain_alive},
    tuio20::{
        Bounds, BoundsEvent, Pointer, PointerEvent, Symbol, SymbolEvent, Token, TokenEvent,
        TuioEvents,
        bundle::{Frame, TuioBundle},
        osc_decoder::OscDecoder,
    },
};

#[derive(Debug, Clone)]
pub struct Processor {
    current_frame: RefCell<Frame>,
    current_time: Cell<TuioTime>,
    pointers: RefCell<HashMap<i32, Pointer>>,
    tokens: RefCell<HashMap<i32, Token>>,
    bounds: RefCell<HashMap<i32, Bounds>>,
    symbols: RefCell<HashMap<i32, Symbol>>,
}

impl TuioProcessor for Processor {
    type Events = TuioEvents;

    fn update(&mut self, packet: OscPacket) -> Option<Self::Events> {
        self.process_packet(packet)
    }
}

impl Processor {
    pub(crate) fn new() -> Self {
        Self {
            current_frame: RefCell::new(Frame::default()),
            current_time: Cell::new(TuioTime::from_system_time().unwrap()),
            pointers: RefCell::new(HashMap::new()),
            tokens: RefCell::new(HashMap::new()),
            bounds: RefCell::new(HashMap::new()),
            symbols: RefCell::new(HashMap::new()),
        }
    }

    fn update_frame(&self, frame: &Frame) -> bool {
        if frame.frame_id() > 0 {
            if frame.frame_id() > self.current_frame.borrow().frame_id() {
                self.current_time.set(TuioTime::from_system_time().unwrap());
            }

            if frame.frame_id() >= self.current_frame.borrow().frame_id()
                || self.current_frame.borrow().frame_id() - frame.frame_id() > 100
            {
                self.current_frame.replace(frame.to_owned());
                return true;
            }
        }

        false
    }

    fn process_packet(&self, packet: OscPacket) -> Option<TuioEvents> {
        if let OscPacket::Bundle(bundle) = packet {
            let tuio_bundle = OscDecoder::decode_bundle(bundle).unwrap();
            let alive = tuio_bundle.alive();
            let current_time = self.current_time.get();
            if self.update_frame(tuio_bundle.frame()) {
                let events = TuioEvents {
                    pointer_events: process_pointers(
                        &mut self.pointers.borrow_mut(),
                        alive,
                        &tuio_bundle,
                        &current_time,
                    ),
                    token_events: process_tokens(
                        &mut self.tokens.borrow_mut(),
                        alive,
                        &tuio_bundle,
                        &current_time,
                    ),
                    bounds_events: process_bounds(
                        &mut self.bounds.borrow_mut(),
                        alive,
                        &tuio_bundle,
                        &current_time,
                    ),
                    symbol_events: process_symbols(
                        &mut self.symbols.borrow_mut(),
                        alive,
                        &tuio_bundle,
                        &current_time,
                    ),
                    frame_event: self.current_frame.borrow().to_owned(),
                };

                return Some(events);
            }
        }
        None
    }
}

fn process_pointers(
    current_pointers: &mut RefMut<HashMap<i32, Pointer>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<PointerEvent> {
    let mut events = Vec::new();
    retain_alive(current_pointers, alive)
        .iter()
        .for_each(|pointer| {
            let event = PointerEvent::Remove(*pointer);
            events.push(event);
        });

    tuio_bundle.pointers().iter().for_each(|pointer| {
        match current_pointers.get_mut(&pointer.session_id()) {
            Some(p) => {
                p.update(current_time, pointer);
                let event = PointerEvent::Update(*p);
                events.push(event);
            }
            None => {
                let session_id = pointer.session_id();
                let new_pointer = Pointer::new(current_time, *pointer);
                current_pointers.insert(session_id, new_pointer);
                let event = PointerEvent::Add(new_pointer);
                events.push(event);
            }
        }
    });
    events
}

fn process_tokens(
    current_tokens: &mut RefMut<HashMap<i32, Token>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<TokenEvent> {
    let mut events = Vec::new();
    retain_alive(current_tokens, alive)
        .iter()
        .for_each(|token| {
            let event = TokenEvent::Remove(*token);
            events.push(event);
        });
    tuio_bundle.tokens().iter().for_each(|token| {
        match current_tokens.get_mut(&token.session_id()) {
            Some(t) => {
                t.update(current_time, token);
                let event = TokenEvent::Update(*t);
                events.push(event);
            }
            None => {
                let session_id = token.session_id();
                let new_token = Token::new(current_time, *token);
                current_tokens.insert(session_id, new_token);
                let event = TokenEvent::Add(new_token);
                events.push(event);
            }
        }
    });
    events
}

fn process_bounds(
    current_bounds: &mut RefMut<HashMap<i32, Bounds>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<BoundsEvent> {
    let mut events = Vec::new();
    retain_alive(current_bounds, alive)
        .iter()
        .for_each(|bounds| {
            let event = BoundsEvent::Remove(*bounds);
            events.push(event);
        });
    tuio_bundle.bounds().iter().for_each(|bounds| {
        match current_bounds.get_mut(&bounds.session_id()) {
            Some(b) => {
                b.update(current_time, bounds);
                let event = BoundsEvent::Update(*b);
                events.push(event);
            }
            None => {
                let session_id = bounds.session_id();
                let new_bounds = Bounds::new(current_time, *bounds);
                current_bounds.insert(session_id, new_bounds);
                let event = BoundsEvent::Add(new_bounds);
                events.push(event);
            }
        }
    });
    events
}

fn process_symbols(
    current_symbols: &mut RefMut<HashMap<i32, Symbol>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<SymbolEvent> {
    let mut events = Vec::new();
    retain_alive(current_symbols, alive)
        .iter()
        .for_each(|symbol| {
            let event = SymbolEvent::Remove(symbol.to_owned());
            events.push(event);
        });
    tuio_bundle.symbols().iter().for_each(|symbol| {
        match current_symbols.get_mut(&symbol.session_id()) {
            Some(s) => {
                s.update(current_time, symbol);
                let event = SymbolEvent::Update(s.to_owned());
                events.push(event);
            }
            None => {
                let session_id = symbol.session_id();
                let new_symbol = Symbol::new(current_time, symbol.to_owned());
                current_symbols.insert(session_id, new_symbol.to_owned());
                let event = SymbolEvent::Add(new_symbol);
                events.push(event);
            }
        }
    });
    events
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
