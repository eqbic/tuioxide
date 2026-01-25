use std::cell::Cell;

use log::{debug, info};
use rosc::OscPacket;
use tuio::{common::tuio_time::TuioTime, tuio20::osc_decoder::OscDecoder};

pub struct Processor {
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time().unwrap()),
        }
    }

    pub fn update(&self, packet: OscPacket) {
        self.process_packet(packet);
    }

    fn process_packet(&self, packet: OscPacket) {
        if let OscPacket::Bundle(bundle) = packet {
            let tuio_bundle = OscDecoder::decode_bundle(bundle).unwrap();
            info!("{:?}", tuio_bundle);
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
