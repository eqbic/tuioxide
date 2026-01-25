use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
};

use log::debug;
use rosc::OscPacket;
use tuio::{
    common::tuio_time::TuioTime,
    tuio11::{
        bundle::{EntityType, TuioBundle, TuioBundleType},
        cursor::Cursor,
        object::Object,
        osc_decoder_encoder::OscDecoder,
    },
};

pub struct Processor {
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
    cursors: RefCell<HashMap<i32, Cursor>>,
    objects: RefCell<HashMap<i32, Object>>,
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time().unwrap()),
            cursors: RefCell::new(HashMap::new()),
            objects: RefCell::new(HashMap::new()),
        }
    }

    pub fn update(&self, packet: OscPacket) {
        self.process_packet(packet);
    }

    pub fn cursors(&self) -> Vec<Cursor> {
        self.cursors.borrow().values().cloned().collect()
    }

    pub fn objects(&self) -> Vec<Object> {
        self.objects.borrow().values().cloned().collect()
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
            if self.update_frame(tuio_bundle.fseq()) {
                match tuio_bundle.profile_type() {
                    TuioBundleType::Cursor => {
                        let mut current_cursors = self.cursors.borrow_mut();
                        process_cursors(&mut current_cursors, alive, &tuio_bundle, &current_time);
                    }
                    TuioBundleType::Object => {
                        let mut current_objects = self.objects.borrow_mut();
                        process_objects(&mut current_objects, alive, &tuio_bundle, &current_time);
                    }
                    TuioBundleType::Blob => {}
                    TuioBundleType::Unknown => {}
                }
            }
        }
    }
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
            debug!("{active_cursor:?}");
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
