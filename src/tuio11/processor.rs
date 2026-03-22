use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
};

use log::{debug, warn};
use rosc::OscPacket;

use crate::{
    core::{TuioTime, processor::TuioProcessor, retain_alive},
    tuio11::{
        Blob, BlobEvent, Cursor, CursorEvent, Object, ObjectEvent, TuioEvents,
        bundle::{EntityType, TuioBundle, TuioBundleType},
        osc_decoder_encoder::OscDecoder,
    },
};

#[derive(Debug, Clone)]
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

impl TuioProcessor for Processor {
    type Events = TuioEvents;

    fn update(&mut self, packet: OscPacket) -> Option<Self::Events> {
        self.process_packet(packet)
    }
}

impl Processor {
    pub(crate) fn new() -> Self {
        Self {
            current_frame: (-1).into(),
            current_time: Cell::new(TuioTime::from_system_time().unwrap()),
            cursors: RefCell::new(HashMap::new()),
            objects: RefCell::new(HashMap::new()),
            blobs: RefCell::new(HashMap::new()),
        }
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
                    current_cursors.insert(session_id, *active_cursor);
                    let event = CursorEvent::Add(*active_cursor);
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
                    current_objects.insert(session_id, *active_object);
                    let event = ObjectEvent::Add(*active_object);
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
                    current_blobs.insert(session_id, *active_blob);
                    let event = BlobEvent::Add(*active_blob);
                    events.push(event);
                }
            };
        }
    }
    events
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};

    use crate::{
        core::processor::TuioProcessor,
        tuio11::{BlobEvent, CursorEvent, ObjectEvent},
    };

    use super::Processor;

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn now_timetag() -> OscTime {
        OscTime::try_from(std::time::SystemTime::now()).unwrap()
    }

    /// Build a `/tuio/2Dcur` bundle with one cursor "set" and the given alive list.
    fn cursor_bundle(
        fseq: i32,
        alive_ids: &[i32],
        sets: &[(i32, f32, f32, f32, f32, f32)], // (session_id, x, y, vx, vy, accel)
    ) -> OscPacket {
        let mut content = Vec::new();

        // source
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![
                OscType::String("source".to_string()),
                OscType::String("test".to_string()),
            ],
        }));

        // alive
        let mut alive_args = vec![OscType::String("alive".to_string())];
        for &id in alive_ids {
            alive_args.push(OscType::Int(id));
        }
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: alive_args,
        }));

        // set messages
        for &(session_id, x, y, vx, vy, accel) in sets {
            content.push(OscPacket::Message(OscMessage {
                addr: "/tuio/2Dcur".to_string(),
                args: vec![
                    OscType::String("set".to_string()),
                    OscType::Int(session_id),
                    OscType::Float(x),
                    OscType::Float(y),
                    OscType::Float(vx),
                    OscType::Float(vy),
                    OscType::Float(accel),
                ],
            }));
        }

        // fseq
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![OscType::String("fseq".to_string()), OscType::Int(fseq)],
        }));

        OscPacket::Bundle(OscBundle {
            timetag: now_timetag(),
            content,
        })
    }

    /// Build a `/tuio/2Dobj` bundle with one object "set".
    #[allow(clippy::too_many_arguments)]
    fn object_bundle(
        fseq: i32,
        alive_ids: &[i32],
        sets: &[(i32, i32, f32, f32, f32, f32, f32, f32, f32, f32)],
        // (session_id, class_id, x, y, angle, vx, vy, rot_speed, accel, rot_accel)
    ) -> OscPacket {
        let mut content = Vec::new();

        let mut alive_args = vec![OscType::String("alive".to_string())];
        for &id in alive_ids {
            alive_args.push(OscType::Int(id));
        }
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: alive_args,
        }));

        for &(session_id, class_id, x, y, angle, vx, vy, rot_speed, accel, rot_accel) in sets {
            content.push(OscPacket::Message(OscMessage {
                addr: "/tuio/2Dobj".to_string(),
                args: vec![
                    OscType::String("set".to_string()),
                    OscType::Int(session_id),
                    OscType::Int(class_id),
                    OscType::Float(x),
                    OscType::Float(y),
                    OscType::Float(angle),
                    OscType::Float(vx),
                    OscType::Float(vy),
                    OscType::Float(rot_speed),
                    OscType::Float(accel),
                    OscType::Float(rot_accel),
                ],
            }));
        }

        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![OscType::String("fseq".to_string()), OscType::Int(fseq)],
        }));

        OscPacket::Bundle(OscBundle {
            timetag: now_timetag(),
            content,
        })
    }

    /// Build a `/tuio/2Dblb` bundle.
    #[allow(clippy::too_many_arguments)]
    fn blob_bundle(
        fseq: i32,
        alive_ids: &[i32],
        // (session_id, x, y, vx, vy, accel, angle, width, height, area, rot_speed, rot_accel)
        sets: &[(i32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)],
    ) -> OscPacket {
        let mut content = Vec::new();

        let mut alive_args = vec![OscType::String("alive".to_string())];
        for &id in alive_ids {
            alive_args.push(OscType::Int(id));
        }
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dblb".to_string(),
            args: alive_args,
        }));

        for &(session_id, x, y, vx, vy, accel, angle, width, height, area, rot_speed, rot_accel) in
            sets
        {
            content.push(OscPacket::Message(OscMessage {
                addr: "/tuio/2Dblb".to_string(),
                args: vec![
                    OscType::String("set".to_string()),
                    OscType::Int(session_id),
                    OscType::Float(x),
                    OscType::Float(y),
                    OscType::Float(vx),
                    OscType::Float(vy),
                    OscType::Float(accel),
                    OscType::Float(angle),
                    OscType::Float(width),
                    OscType::Float(height),
                    OscType::Float(area),
                    OscType::Float(rot_speed),
                    OscType::Float(rot_accel),
                ],
            }));
        }

        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dblb".to_string(),
            args: vec![OscType::String("fseq".to_string()), OscType::Int(fseq)],
        }));

        OscPacket::Bundle(OscBundle {
            timetag: now_timetag(),
            content,
        })
    }

    // ── Non-bundle input ──────────────────────────────────────────────────────

    #[test]
    fn non_bundle_packet_returns_none() {
        let mut proc = Processor::new();
        let msg = OscPacket::Message(OscMessage {
            addr: "/something".to_string(),
            args: vec![],
        });
        let result = proc.update(msg);
        assert!(result.is_none(), "a bare OscMessage should return None");
    }

    // ── Cursor add event ──────────────────────────────────────────────────────

    #[test]
    fn first_cursor_produces_add_event() {
        let mut proc = Processor::new();
        let packet = cursor_bundle(1, &[10], &[(10, 0.5, 0.5, 0.0, 0.0, 0.0)]);
        let events = proc.update(packet).expect("should return events");
        assert_eq!(events.cursor_events.len(), 1);
        assert!(
            matches!(events.cursor_events[0], CursorEvent::Add(_)),
            "expected CursorEvent::Add"
        );
    }

    #[test]
    fn add_event_carries_correct_session_id() {
        let mut proc = Processor::new();
        let packet = cursor_bundle(1, &[42], &[(42, 0.3, 0.7, 0.0, 0.0, 0.0)]);
        let events = proc.update(packet).unwrap();
        if let CursorEvent::Add(cursor) = &events.cursor_events[0] {
            assert_eq!(cursor.session_id(), 42);
        } else {
            panic!("expected CursorEvent::Add");
        }
    }

    #[test]
    fn add_event_carries_correct_position() {
        let mut proc = Processor::new();
        let packet = cursor_bundle(1, &[1], &[(1, 0.25, 0.75, 0.0, 0.0, 0.0)]);
        let events = proc.update(packet).unwrap();
        if let CursorEvent::Add(cursor) = &events.cursor_events[0] {
            assert_relative_eq!(cursor.position().x, 0.25);
            assert_relative_eq!(cursor.position().y, 0.75);
        } else {
            panic!("expected CursorEvent::Add");
        }
    }

    #[test]
    fn multiple_cursors_all_produce_add_events() {
        let mut proc = Processor::new();
        let packet = cursor_bundle(
            1,
            &[1, 2],
            &[(1, 0.1, 0.1, 0.0, 0.0, 0.0), (2, 0.9, 0.9, 0.0, 0.0, 0.0)],
        );
        let events = proc.update(packet).unwrap();
        assert_eq!(events.cursor_events.len(), 2);
        for ev in &events.cursor_events {
            assert!(matches!(ev, CursorEvent::Add(_)));
        }
    }

    // ── Cursor update event ───────────────────────────────────────────────────

    #[test]
    fn second_frame_same_cursor_produces_update_event() {
        let mut proc = Processor::new();

        // Frame 1: add cursor 5
        let p1 = cursor_bundle(1, &[5], &[(5, 0.1, 0.1, 0.0, 0.0, 0.0)]);
        proc.update(p1).unwrap();

        // Frame 2: same cursor 5, different position
        let p2 = cursor_bundle(2, &[5], &[(5, 0.9, 0.9, 0.1, 0.2, 0.0)]);
        let events = proc.update(p2).unwrap();

        assert_eq!(events.cursor_events.len(), 1);
        assert!(
            matches!(events.cursor_events[0], CursorEvent::Update(_)),
            "expected CursorEvent::Update on second frame"
        );
    }

    #[test]
    fn update_event_carries_new_session_id() {
        let mut proc = Processor::new();
        proc.update(cursor_bundle(1, &[5], &[(5, 0.0, 0.0, 0.0, 0.0, 0.0)]))
            .unwrap();
        let events = proc
            .update(cursor_bundle(2, &[5], &[(5, 0.5, 0.5, 0.0, 0.0, 0.0)]))
            .unwrap();
        if let CursorEvent::Update(cursor) = &events.cursor_events[0] {
            assert_eq!(cursor.session_id(), 5);
        } else {
            panic!("expected CursorEvent::Update");
        }
    }

    // ── Cursor remove event ───────────────────────────────────────────────────

    #[test]
    fn cursor_absent_from_alive_produces_remove_event() {
        let mut proc = Processor::new();

        // Frame 1: cursor 3 is alive
        proc.update(cursor_bundle(1, &[3], &[(3, 0.5, 0.5, 0.0, 0.0, 0.0)]))
            .unwrap();

        // Frame 2: alive list is empty — cursor 3 should be removed
        let events = proc.update(cursor_bundle(2, &[], &[])).unwrap();

        assert_eq!(events.cursor_events.len(), 1);
        assert!(
            matches!(events.cursor_events[0], CursorEvent::Remove(_)),
            "expected CursorEvent::Remove when cursor disappears from alive"
        );
    }

    #[test]
    fn remove_event_carries_correct_session_id() {
        let mut proc = Processor::new();
        proc.update(cursor_bundle(1, &[7], &[(7, 0.0, 0.0, 0.0, 0.0, 0.0)]))
            .unwrap();
        let events = proc.update(cursor_bundle(2, &[], &[])).unwrap();
        if let CursorEvent::Remove(cursor) = &events.cursor_events[0] {
            assert_eq!(cursor.session_id(), 7);
        } else {
            panic!("expected CursorEvent::Remove");
        }
    }

    #[test]
    fn partial_alive_keeps_alive_cursors_and_removes_dead_ones() {
        let mut proc = Processor::new();

        // Frame 1: add cursors 1 and 2
        proc.update(cursor_bundle(
            1,
            &[1, 2],
            &[(1, 0.1, 0.1, 0.0, 0.0, 0.0), (2, 0.9, 0.9, 0.0, 0.0, 0.0)],
        ))
        .unwrap();

        // Frame 2: only cursor 1 is alive → cursor 2 should be removed, cursor 1 updated
        let events = proc
            .update(cursor_bundle(2, &[1], &[(1, 0.2, 0.2, 0.0, 0.0, 0.0)]))
            .unwrap();

        let removes: Vec<_> = events
            .cursor_events
            .iter()
            .filter(|e| matches!(e, CursorEvent::Remove(_)))
            .collect();
        let updates: Vec<_> = events
            .cursor_events
            .iter()
            .filter(|e| matches!(e, CursorEvent::Update(_)))
            .collect();

        assert_eq!(removes.len(), 1, "exactly one cursor should be removed");
        assert_eq!(updates.len(), 1, "exactly one cursor should be updated");

        if let CursorEvent::Remove(removed) = removes[0] {
            assert_eq!(removed.session_id(), 2);
        }
        if let CursorEvent::Update(updated) = updates[0] {
            assert_eq!(updated.session_id(), 1);
        }
    }

    // ── Duplicate / stale frame ───────────────────────────────────────────────

    #[test]
    fn frame_zero_returns_none() {
        let mut proc = Processor::new();
        // fseq = 0 is never accepted (update_frame requires frame > 0)
        let result = proc.update(cursor_bundle(0, &[1], &[(1, 0.0, 0.0, 0.0, 0.0, 0.0)]));
        assert!(result.is_none(), "frame 0 should return None");
    }

    #[test]
    fn negative_frame_returns_none() {
        let mut proc = Processor::new();
        let result = proc.update(cursor_bundle(-5, &[1], &[(1, 0.0, 0.0, 0.0, 0.0, 0.0)]));
        assert!(result.is_none(), "negative frame should return None");
    }

    #[test]
    fn stale_frame_close_to_current_returns_none() {
        let mut proc = Processor::new();

        // Accept frame 50
        proc.update(cursor_bundle(50, &[1], &[(1, 0.0, 0.0, 0.0, 0.0, 0.0)]))
            .unwrap();

        // Frame 49 (old, within 100-frame window) should be rejected
        let result = proc.update(cursor_bundle(49, &[1], &[(1, 0.5, 0.5, 0.0, 0.0, 0.0)]));
        assert!(
            result.is_none(),
            "frame 49 after frame 50 should return None"
        );
    }

    #[test]
    fn very_old_frame_wraps_around_and_is_accepted() {
        let mut proc = Processor::new();

        // Accept frame 200
        proc.update(cursor_bundle(200, &[1], &[(1, 0.0, 0.0, 0.0, 0.0, 0.0)]))
            .unwrap();

        // Frame 99 is 101 frames behind current (200); difference > 100, so it should be accepted.
        let result = proc.update(cursor_bundle(99, &[1], &[(1, 0.5, 0.5, 0.0, 0.0, 0.0)]));
        assert!(
            result.is_some(),
            "frame more than 100 behind current should wrap and be accepted"
        );
    }

    // ── Object events ─────────────────────────────────────────────────────────

    #[test]
    fn first_object_produces_add_event() {
        let mut proc = Processor::new();
        let packet = object_bundle(1, &[1], &[(1, 5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)]);
        let events = proc.update(packet).unwrap();
        assert_eq!(events.object_events.len(), 1);
        assert!(matches!(events.object_events[0], ObjectEvent::Add(_)));
    }

    #[test]
    fn object_update_event_on_second_frame() {
        let mut proc = Processor::new();
        proc.update(object_bundle(
            1,
            &[1],
            &[(1, 5, 0.1, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)],
        ))
        .unwrap();
        let events = proc
            .update(object_bundle(
                2,
                &[1],
                &[(1, 5, 0.9, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)],
            ))
            .unwrap();
        assert!(matches!(events.object_events[0], ObjectEvent::Update(_)));
    }

    #[test]
    fn object_remove_event_when_absent_from_alive() {
        let mut proc = Processor::new();
        proc.update(object_bundle(
            1,
            &[1],
            &[(1, 3, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)],
        ))
        .unwrap();
        let events = proc.update(object_bundle(2, &[], &[])).unwrap();
        assert_eq!(events.object_events.len(), 1);
        assert!(matches!(events.object_events[0], ObjectEvent::Remove(_)));
    }

    // ── Blob events ───────────────────────────────────────────────────────────

    #[test]
    fn first_blob_produces_add_event() {
        let mut proc = Processor::new();
        let packet = blob_bundle(
            1,
            &[1],
            &[(1, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.2, 0.3, 0.06, 0.0, 0.0)],
        );
        let events = proc.update(packet).unwrap();
        assert_eq!(events.blob_events.len(), 1);
        assert!(matches!(events.blob_events[0], BlobEvent::Add(_)));
    }

    #[test]
    fn blob_update_event_on_second_frame() {
        let mut proc = Processor::new();
        proc.update(blob_bundle(
            1,
            &[1],
            &[(1, 0.1, 0.1, 0.0, 0.0, 0.0, 0.0, 0.2, 0.3, 0.06, 0.0, 0.0)],
        ))
        .unwrap();
        let events = proc
            .update(blob_bundle(
                2,
                &[1],
                &[(1, 0.9, 0.9, 0.1, 0.2, 0.0, 0.0, 0.2, 0.3, 0.06, 0.0, 0.0)],
            ))
            .unwrap();
        assert!(matches!(events.blob_events[0], BlobEvent::Update(_)));
    }

    #[test]
    fn blob_remove_event_when_absent_from_alive() {
        let mut proc = Processor::new();
        proc.update(blob_bundle(
            1,
            &[1],
            &[(1, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.2, 0.3, 0.06, 0.0, 0.0)],
        ))
        .unwrap();
        let events = proc.update(blob_bundle(2, &[], &[])).unwrap();
        assert_eq!(events.blob_events.len(), 1);
        assert!(matches!(events.blob_events[0], BlobEvent::Remove(_)));
    }

    // ── Events contain no cross-contamination ─────────────────────────────────

    #[test]
    fn cursor_bundle_does_not_produce_object_or_blob_events() {
        let mut proc = Processor::new();
        let events = proc
            .update(cursor_bundle(1, &[1], &[(1, 0.5, 0.5, 0.0, 0.0, 0.0)]))
            .unwrap();
        assert!(
            events.object_events.is_empty(),
            "cursor bundle should not produce object events"
        );
        assert!(
            events.blob_events.is_empty(),
            "cursor bundle should not produce blob events"
        );
    }

    #[test]
    fn object_bundle_does_not_produce_cursor_or_blob_events() {
        let mut proc = Processor::new();
        let events = proc
            .update(object_bundle(
                1,
                &[1],
                &[(1, 0, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)],
            ))
            .unwrap();
        assert!(events.cursor_events.is_empty());
        assert!(events.blob_events.is_empty());
    }

    // ── Increasing monotonic frames always accepted ───────────────────────────

    #[test]
    fn monotonically_increasing_frames_all_accepted() {
        let mut proc = Processor::new();
        for fseq in 1..=10 {
            let packet = cursor_bundle(fseq, &[1], &[(1, 0.0, 0.0, 0.0, 0.0, 0.0)]);
            let result = proc.update(packet);
            assert!(result.is_some(), "frame {fseq} should be accepted");
        }
    }
}
