use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rosc::OscPacket;

use crate::{
    common::{osc_receiver::OscReceiver, tuio_time::TuioTime},
    tuio11::{
        self,
        cursor::Cursor,
        object::Object,
        osc_decoder_encoder::{EntityType, OscDecoder, TuioBundle},
        profile::Profile,
    },
};

pub struct Client<R>
where
    R: OscReceiver<OscPacket> + Send + Sync + 'static,
{
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
    cursors: RefCell<HashMap<i32, Cursor>>,
    objects: RefCell<HashMap<i32, Object>>,
    receiver: Arc<R>,
    buffer: Arc<Mutex<ConstGenericRingBuffer<OscPacket, 64>>>,
}

fn retain_alive<T>(current: &mut HashMap<i32, T>, alive: &HashSet<i32>) -> Vec<T>
where
    T: Copy,
{
    let to_remove: Vec<T> = current
        .iter()
        .filter(|(key, _)| !alive.contains(key))
        .map(|(_, v)| *v)
        .collect();
    current.retain(|key, _| alive.contains(key));
    to_remove
}

fn process_cursors(
    current_cursors: &mut RefMut<HashMap<i32, Cursor>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) {
    retain_alive(current_cursors, alive);
    if let Some(EntityType::Cursor(cursors)) = tuio_bundle.tuio_entities() {
        for active_cursor in cursors {
            match current_cursors.get_mut(&active_cursor.session_id()) {
                Some(cursor) => {
                    cursor.update(current_time, active_cursor);
                }
                None => {
                    let session_id = active_cursor.session_id();
                    let new_cursor = Cursor::new(current_time, *active_cursor);
                    current_cursors.insert(session_id, new_cursor);
                }
            };
        }
    }
}

fn process_objects(
    current_objects: &mut RefMut<HashMap<i32, Object>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) {
    retain_alive(current_objects, alive);
    if let Some(EntityType::Object(objects)) = tuio_bundle.tuio_entities() {
        for active_object in objects {
            match current_objects.get_mut(&active_object.session_id()) {
                Some(object) => {
                    object.update(current_time, active_object);
                }
                None => {
                    let session_id = active_object.session_id();
                    let new_object = Object::new(current_time, *active_object);
                    current_objects.insert(session_id, new_object);
                }
            };
        }
    }
}

impl<R> Client<R>
where
    R: OscReceiver<OscPacket> + Send + Sync + 'static,
{
    pub fn new(remote: std::net::Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time()?),
            cursors: RefCell::new(HashMap::new()),
            objects: RefCell::new(HashMap::new()),
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

    pub fn cursors(&self) -> Vec<Cursor> {
        self.cursors.borrow().values().cloned().collect()
    }

    pub fn objects(&self) -> Vec<Object> {
        self.objects.borrow().values().cloned().collect()
    }

    fn process_packet(&self, packet: OscPacket) -> anyhow::Result<()> {
        if let OscPacket::Bundle(bundle) = packet {
            let tuio_bundle = OscDecoder::decode_bundle(bundle)?;
            let alive: HashSet<i32> = tuio_bundle.alive().iter().copied().collect();
            let current_time = self.current_time.get();
            if self.update_frame(tuio_bundle.fseq())? {
                match tuio_bundle.profile_type() {
                    tuio11::osc_decoder_encoder::TuioBundleType::Cursor => {
                        let mut current_cursors = self.cursors.borrow_mut();
                        process_cursors(&mut current_cursors, &alive, &tuio_bundle, &current_time);
                    }
                    tuio11::osc_decoder_encoder::TuioBundleType::Object => {
                        let mut current_objects = self.objects.borrow_mut();
                        process_objects(&mut current_objects, &alive, &tuio_bundle, &current_time);
                    }
                    tuio11::osc_decoder_encoder::TuioBundleType::Blob => {}
                    tuio11::osc_decoder_encoder::TuioBundleType::Unknown => {}
                }
            }
        }

        Ok(())
    }
}
