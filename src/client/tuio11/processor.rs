use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
};

use log::{debug, warn};
use rosc::OscPacket;

use crate::core::{
    profile::Profile,
    tuio_time::TuioTime,
    tuio11::{
        blob::Blob,
        bundle::{EntityType, TuioBundle, TuioBundleType},
        cursor::Cursor,
        event::{BlobEvent, CursorEvent, ObjectEvent},
        object::Object,
        osc_decoder_encoder::OscDecoder,
    },
    utils::retain_alive,
};

#[derive(Debug, Default)]
pub struct TuioEvents {
    pub cursor_events: Vec<CursorEvent>,
    pub object_events: Vec<ObjectEvent>,
    pub blob_events: Vec<BlobEvent>,
}

pub struct Processor {
    current_frame: Cell<i32>,
    current_time: Cell<TuioTime>,
    cursors: RefCell<HashMap<i32, Cursor>>,
    objects: RefCell<HashMap<i32, Object>>,
    blobs: RefCell<HashMap<i32, Blob>>,
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
            blobs: RefCell::new(HashMap::new()),
        }
    }

    pub fn update(&self, packet: OscPacket) -> Option<TuioEvents> {
        self.process_packet(packet)
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

    fn process_packet(&self, packet: OscPacket) -> Option<TuioEvents> {
        if let OscPacket::Bundle(bundle) = packet {
            let tuio_bundle = OscDecoder::decode_bundle(bundle).unwrap();
            let alive = tuio_bundle.alive();
            let current_time = self.current_time.get();
            if self.update_frame(tuio_bundle.fseq()) {
                let mut events = TuioEvents::default();
                match tuio_bundle.profile_type() {
                    TuioBundleType::Cursor => {
                        let mut current_cursors = self.cursors.borrow_mut();
                        events.cursor_events = process_cursors(
                            &mut current_cursors,
                            alive,
                            &tuio_bundle,
                            &current_time,
                        );
                    }
                    TuioBundleType::Object => {
                        let mut current_objects = self.objects.borrow_mut();
                        events.object_events = process_objects(
                            &mut current_objects,
                            alive,
                            &tuio_bundle,
                            &current_time,
                        );
                    }
                    TuioBundleType::Blob => {
                        let mut current_blobs = self.blobs.borrow_mut();
                        events.blob_events =
                            process_blobs(&mut current_blobs, alive, &tuio_bundle, &current_time);
                    }
                    TuioBundleType::Unknown => {
                        warn!("Unknown Tuio Bundle Type")
                    }
                }
                return Some(events);
            }
        }
        None
    }
}

fn process_cursors(
    current_cursors: &mut RefMut<HashMap<i32, Cursor>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<CursorEvent> {
    let mut events = Vec::new();
    retain_alive(current_cursors, alive)
        .iter()
        .for_each(|cursor| {
            let event = CursorEvent::Remove(*cursor);
            events.push(event);
        });
    if let Some(EntityType::Cursor(cursors)) = tuio_bundle.tuio_entities() {
        for active_cursor in cursors {
            debug!("{active_cursor:?}");
            match current_cursors.get_mut(&active_cursor.session_id()) {
                Some(cursor) => {
                    cursor.update(current_time, active_cursor);
                    let event = CursorEvent::Update(*cursor);
                    events.push(event);
                }
                None => {
                    let session_id = active_cursor.session_id();
                    let new_cursor = Cursor::new(current_time, *active_cursor);
                    current_cursors.insert(session_id, new_cursor);
                    let event = CursorEvent::Add(new_cursor);
                    events.push(event);
                }
            };
        }
    }
    events
}

fn process_objects(
    current_objects: &mut RefMut<HashMap<i32, Object>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<ObjectEvent> {
    let mut events = Vec::new();
    retain_alive(current_objects, alive)
        .iter()
        .for_each(|object| {
            let event = ObjectEvent::Remove(*object);
            events.push(event);
        });
    if let Some(EntityType::Object(objects)) = tuio_bundle.tuio_entities() {
        for active_object in objects {
            match current_objects.get_mut(&active_object.session_id()) {
                Some(object) => {
                    object.update(current_time, active_object);
                    let event = ObjectEvent::Update(*object);
                    events.push(event);
                }
                None => {
                    let session_id = active_object.session_id();
                    let new_object = Object::new(current_time, *active_object);
                    current_objects.insert(session_id, new_object);
                    let event = ObjectEvent::Add(new_object);
                    events.push(event);
                }
            };
        }
    }
    events
}

fn process_blobs(
    current_blobs: &mut RefMut<HashMap<i32, Blob>>,
    alive: &HashSet<i32>,
    tuio_bundle: &TuioBundle,
    current_time: &TuioTime,
) -> Vec<BlobEvent> {
    let mut events = Vec::new();
    retain_alive(current_blobs, alive).iter().for_each(|blob| {
        let event = BlobEvent::Remove(*blob);
        events.push(event);
    });
    if let Some(EntityType::Blob(blobs)) = tuio_bundle.tuio_entities() {
        for active_blob in blobs {
            match current_blobs.get_mut(&active_blob.session_id()) {
                Some(blob) => {
                    blob.update(current_time, active_blob);
                    let event = BlobEvent::Update(*blob);
                    events.push(event);
                }
                None => {
                    let session_id = active_blob.session_id();
                    let new_blob = Blob::new(current_time, *active_blob);
                    current_blobs.insert(session_id, new_blob);
                    let event = BlobEvent::Add(new_blob);
                    events.push(event);
                }
            };
        }
    }
    events
}
