use std::collections::HashMap;

use rosc::OscBundle;

use crate::{core::TuioProfile, tuio11::osc_decoder_encoder::OscEncoder};

pub(crate) struct TuioRepository<P: TuioProfile> {
    source: String,
    entities: HashMap<i32, P>,
}

impl<P: TuioProfile> TuioRepository<P> {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
            entities: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: P) {
        self.entities.insert(entity.session_id(), entity);
    }

    pub fn update(&mut self, entity: P) {
        if let Some(e) = self.entities.get_mut(&entity.session_id()) {
            *e = entity
        }
    }

    pub fn remove(&mut self, session_id: i32) {
        self.entities.remove(&session_id);
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn bundle(&self, frame_id: i32) -> OscBundle {
        OscEncoder::encode_bundle(self.entities.values().cloned(), &self.source, frame_id)
    }
}

#[cfg(test)]
mod tests {
    use rosc::{OscPacket, OscType};

    use crate::{
        core::{Position, Velocity},
        tuio11::{Cursor, Object, osc_decoder_encoder::OscDecoder},
    };

    use super::TuioRepository;

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn make_cursor(session_id: i32, x: f32, y: f32) -> Cursor {
        Cursor::new(
            session_id,
            Position::new(x, y),
            Velocity::new(0.0, 0.0),
            0.0,
        )
    }

    fn make_object(session_id: i32, class_id: i32) -> Object {
        Object::new(
            session_id,
            class_id,
            Position::new(0.5, 0.5),
            Velocity::new(0.0, 0.0),
            0.0,
            0.0,
            0.0,
            0.0,
        )
    }

    // ── add ───────────────────────────────────────────────────────────────────

    #[test]
    fn add_single_entity_is_present_in_bundle() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.2, 0.3));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&1));
    }

    #[test]
    fn add_multiple_entities_all_present_in_alive() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(10, 0.1, 0.1));
        repo.add(make_cursor(20, 0.5, 0.5));
        repo.add(make_cursor(30, 0.9, 0.9));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&10));
        assert!(decoded.alive().contains(&20));
        assert!(decoded.alive().contains(&30));
    }

    #[test]
    fn add_same_session_id_twice_overwrites() {
        // Adding the same session_id twice replaces the first — only one entity remains.
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(5, 0.1, 0.1));
        repo.add(make_cursor(5, 0.9, 0.9));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        // alive set should contain 5 exactly once
        assert!(decoded.alive().contains(&5));
        assert_eq!(decoded.alive().len(), 1);
    }

    #[test]
    fn add_object_entity_appears_in_alive() {
        let mut repo: TuioRepository<Object> = TuioRepository::new("test");
        repo.add(make_object(99, 3));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&99));
    }

    // ── update ────────────────────────────────────────────────────────────────

    #[test]
    fn update_existing_entity_replaces_it() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(7, 0.1, 0.2));
        // Now update session 7 to a new position
        repo.update(make_cursor(7, 0.8, 0.9));
        // Entity with id 7 should still be present (not duplicated)
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&7));
        assert_eq!(decoded.alive().len(), 1);
    }

    #[test]
    fn update_nonexistent_entity_is_a_no_op() {
        // update() only modifies existing entries; calling it for an unknown ID
        // should leave the repository unchanged.
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.update(make_cursor(42, 0.5, 0.5));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        // session 42 was never added, so alive should be empty
        assert!(!decoded.alive().contains(&42));
    }

    // ── remove ────────────────────────────────────────────────────────────────

    #[test]
    fn remove_existing_entity_is_absent_from_bundle() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(3, 0.5, 0.5));
        repo.remove(3);
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(!decoded.alive().contains(&3));
    }

    #[test]
    fn remove_one_of_many_leaves_others_intact() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.1, 0.1));
        repo.add(make_cursor(2, 0.5, 0.5));
        repo.add(make_cursor(3, 0.9, 0.9));
        repo.remove(2);
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&1));
        assert!(!decoded.alive().contains(&2));
        assert!(decoded.alive().contains(&3));
    }

    #[test]
    fn remove_nonexistent_id_is_harmless() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.5, 0.5));
        // Removing a session id that was never added should not panic or corrupt state.
        repo.remove(999);
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&1));
        assert_eq!(decoded.alive().len(), 1);
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_removes_all_entities() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.1, 0.1));
        repo.add(make_cursor(2, 0.5, 0.5));
        repo.add(make_cursor(3, 0.9, 0.9));
        repo.clear();
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(
            decoded.alive().is_empty(),
            "all entities should be gone after clear"
        );
    }

    #[test]
    fn clear_on_empty_repo_is_harmless() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.clear(); // should not panic
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().is_empty());
    }

    #[test]
    fn add_after_clear_works_correctly() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.1, 0.1));
        repo.clear();
        repo.add(make_cursor(2, 0.9, 0.9));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(!decoded.alive().contains(&1));
        assert!(decoded.alive().contains(&2));
    }

    // ── bundle ────────────────────────────────────────────────────────────────

    #[test]
    fn bundle_encodes_the_given_frame_id() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.5, 0.5));
        let bundle = repo.bundle(77);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.fseq(), 77);
    }

    #[test]
    fn bundle_encodes_the_source_name() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("mysource");
        repo.add(make_cursor(1, 0.5, 0.5));
        let bundle = repo.bundle(1);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.source(), &Some("mysource".to_string()));
    }

    #[test]
    fn bundle_frame_id_zero_encodes_correctly() {
        let repo: TuioRepository<Cursor> = TuioRepository::new("test");
        let bundle = repo.bundle(0);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.fseq(), 0);
    }

    #[test]
    fn bundle_all_content_items_are_messages() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.1, 0.2));
        let bundle = repo.bundle(5);
        for item in &bundle.content {
            assert!(
                matches!(item, OscPacket::Message(_)),
                "all bundle content items should be OscPacket::Message"
            );
        }
    }

    #[test]
    fn bundle_fseq_message_is_last_content_item() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.5, 0.5));
        let bundle = repo.bundle(42);
        let last = bundle.content.last().expect("bundle should not be empty");
        if let OscPacket::Message(msg) = last {
            assert_eq!(
                msg.args.get(1),
                Some(&OscType::Int(42)),
                "last message must be fseq with the given frame_id"
            );
        } else {
            panic!("expected OscPacket::Message as last content item");
        }
    }

    #[test]
    fn bundle_different_frame_ids_produce_different_fseq() {
        let mut repo: TuioRepository<Cursor> = TuioRepository::new("test");
        repo.add(make_cursor(1, 0.5, 0.5));

        let b1 = repo.bundle(10);
        let b2 = repo.bundle(20);

        let d1 = OscDecoder::decode_bundle(b1).unwrap();
        let d2 = OscDecoder::decode_bundle(b2).unwrap();

        assert_eq!(d1.fseq(), 10);
        assert_eq!(d2.fseq(), 20);
    }
}
