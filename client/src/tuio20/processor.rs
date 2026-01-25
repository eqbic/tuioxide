use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
    hash::Hash,
};

use log::{debug, info};
use rosc::OscPacket;
use tuio::{
    common::{tuio_time::TuioTime, utils::retain_alive},
    tuio20::{
        bounds::Bounds, bundle::TuioBundle, osc_decoder::OscDecoder, pointer::Pointer,
        symbol::Symbol, token::Token,
    },
};

pub struct Processor {
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
    pointers: RefCell<HashMap<i32, Pointer>>,
    tokens: RefCell<HashMap<i32, Token>>,
    bounds: RefCell<HashMap<i32, Bounds>>,
    symbols: RefCell<HashMap<i32, Symbol>>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time().unwrap()),
            pointers: RefCell::new(HashMap::new()),
            tokens: RefCell::new(HashMap::new()),
            bounds: RefCell::new(HashMap::new()),
            symbols: RefCell::new(HashMap::new()),
        }
    }

    pub fn pointers(&self) -> Vec<Pointer> {
        self.pointers.borrow().values().cloned().collect()
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.borrow().values().cloned().collect()
    }

    pub fn update(&self, packet: OscPacket) {
        self.process_packet(packet);
    }

    fn update_frame(&self, frame: i32) -> bool {
        if frame > 0 {
            if frame > self.current_frame.get() {
                self.current_time.set(TuioTime::from_system_time().unwrap());
            }

            if frame >= self.current_frame.get() || self.current_frame.get() - frame > 100 {
                self.current_frame.set(frame);
                return true;
            }
        }

        false
    }

    fn process_packet(&self, packet: OscPacket) {
        if let OscPacket::Bundle(bundle) = packet {
            let tuio_bundle = OscDecoder::decode_bundle(bundle).unwrap();
            let alive = tuio_bundle.alive();
            let current_time = self.current_time.get();
            if self.update_frame(tuio_bundle.frame().frame_id()) {
                process_pointers(
                    &mut self.pointers.borrow_mut(),
                    alive,
                    &tuio_bundle,
                    &current_time,
                );

                process_tokens(
                    &mut self.tokens.borrow_mut(),
                    alive,
                    &tuio_bundle,
                    &current_time,
                );
            }
        }
    }
}

fn process_pointers(
    current_pointers: &mut RefMut<HashMap<i32, Pointer>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) {
    retain_alive(current_pointers, alive);
    tuio_bundle.pointers().iter().for_each(|pointer| {
        match current_pointers.get_mut(&pointer.session_id()) {
            Some(p) => p.update(current_time, pointer),
            None => {
                let session_id = pointer.session_id();
                let new_pointer = Pointer::new(current_time, *pointer);
                current_pointers.insert(session_id, new_pointer);
            }
        }
    });
}

fn process_tokens(
    current_tokens: &mut RefMut<HashMap<i32, Token>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) {
    retain_alive(current_tokens, alive);
    tuio_bundle.tokens().iter().for_each(|token| {
        match current_tokens.get_mut(&token.session_id()) {
            Some(t) => t.update(current_time, token),
            None => {
                let session_id = token.session_id();
                let new_pointer = Token::new(current_time, *token);
                current_tokens.insert(session_id, new_pointer);
            }
        }
    });
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
