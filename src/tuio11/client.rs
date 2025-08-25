use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rosc::OscPacket;

use crate::{
    common::{
        osc_receiver::{OscReceiver, UdpReceiver},
        tuio_time::TuioTime,
    },
    tuio11::{self, blob::Blob, cursor::Cursor, object::Object, osc_decoder_encoder::OscDecoder},
};

pub struct Client {
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
    cursors: HashMap<i32, Cursor>,
    receiver: Arc<UdpReceiver>,
    buffer: Arc<Mutex<ConstGenericRingBuffer<OscPacket, 64>>>,
}

impl Client {
    pub fn new(remote: std::net::Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time()?),
            cursors: HashMap::new(),
            receiver: Arc::new(UdpReceiver::new(remote, port)?),
            buffer: Default::default(),
        })
    }

    pub fn connect(&self) -> anyhow::Result<()> {
        self.receiver.connect()?;
        let receiver = self.receiver.clone();
        let buffer = self.buffer.clone();
        thread::spawn(move || {
            while let Ok(packet) = receiver.recv() {
                buffer.lock().unwrap().enqueue(packet);
            }
        });
        Ok(())
    }

    pub fn disconnect(&self) -> anyhow::Result<()> {
        self.receiver.disconnect()
    }

    pub fn update(&self) -> anyhow::Result<()> {
        for packet in self.buffer.lock().unwrap().drain() {
            self.process_packet(packet)?;
        }
        Ok(())
    }

    fn update_frame(&self, frame: i32) -> anyhow::Result<bool> {
        if frame > 0 {
            if frame > self.current_frame.get() {
                self.current_time.set(TuioTime::from_system_time()?);
            }

            if frame >= self.current_frame.get() || self.current_frame.get() - frame > 100 {
                self.current_frame.set(frame);
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn process_packet(&self, packet: OscPacket) -> anyhow::Result<()> {
        if let OscPacket::Bundle(bundle) = packet {
            let tuio_bundle = OscDecoder::decode_bundle(bundle)?;
            println!("{:?}", tuio_bundle);
            let alive: HashSet<i32> = tuio_bundle.alive().iter().copied().collect();
            if self.update_frame(tuio_bundle.fseq())? {
                match tuio_bundle.profile_type() {
                    tuio11::osc_decoder_encoder::TuioBundleType::Cursor => {}
                    tuio11::osc_decoder_encoder::TuioBundleType::Object => {}
                    tuio11::osc_decoder_encoder::TuioBundleType::Blob => {}
                    tuio11::osc_decoder_encoder::TuioBundleType::Unknown => {}
                }
            }
        }

        Ok(())
    }
}
