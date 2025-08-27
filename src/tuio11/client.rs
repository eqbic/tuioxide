use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rosc::OscPacket;

use crate::{
    common::{osc_receiver::OscReceiver, tuio_time::TuioTime},
    tuio11::{self, cursor::Cursor, osc_decoder_encoder::OscDecoder},
};

pub struct Client<R>
where
    R: OscReceiver<OscPacket> + Send + Sync + 'static,
{
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
    cursors: HashMap<i32, Cursor>,
    receiver: Arc<R>,
    buffer: Arc<Mutex<ConstGenericRingBuffer<OscPacket, 64>>>,
}

impl<R> Client<R>
where
    R: OscReceiver<OscPacket> + Send + Sync + 'static,
{
    pub fn new(remote: std::net::Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time()?),
            cursors: HashMap::new(),
            receiver: Arc::new(R::connect(remote, port)?),
            buffer: Default::default(),
        })
    }

    pub fn connect(&self) -> anyhow::Result<()> {
        let receiver = self.receiver.clone();
        let buffer = self.buffer.clone();
        thread::spawn(move || {
            while let Ok(packet) = receiver.recv() {
                buffer.lock().unwrap().enqueue(packet);
            }
        });

        Ok(())
    }

    pub fn disconnect(&self) {
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
