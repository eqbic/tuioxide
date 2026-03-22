use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};

use std::{iter, time::SystemTime};

use crate::core::TuioProfile;

use crate::core::TuioError;
use crate::tuio11::bundle::{TuioBundle, TuioBundleType};

pub(crate) struct OscDecoder;

impl OscDecoder {
    pub(crate) fn decode_bundle(bundle: OscBundle) -> Result<TuioBundle, TuioError> {
        let mut tuio_bundle = TuioBundle::default();
        for packet in &bundle.content {
            if let OscPacket::Message(message) = packet {
                let tuio_type = match message.addr.as_str() {
                    "/tuio/2Dcur" => TuioBundleType::Cursor,
                    "/tuio/2Dobj" => TuioBundleType::Object,
                    "/tuio/2Dblb" => TuioBundleType::Blob,
                    _ => return Err(TuioError::UnknownAddress(message.clone())),
                };

                tuio_bundle.set_type(tuio_type);

                match message.args.first() {
                    Some(OscType::String(arg)) => match arg.as_str() {
                        "source" => tuio_bundle.set_source(message),
                        "alive" => tuio_bundle.set_alive(message),
                        "set" => tuio_bundle.set_set(message)?,
                        "fseq" => tuio_bundle.set_fseq(message)?,
                        _ => return Err(TuioError::UnknownMessageType(message.clone())),
                    },
                    None => return Err(TuioError::EmptyMessage(message.clone())),
                    _ => return Err(TuioError::UnknownMessageType(message.clone())),
                }
            }
        }
        Ok(tuio_bundle)
    }
}

pub(crate) struct OscEncoder;

impl OscEncoder {
    pub(crate) fn encode_bundle<E, I>(
        profile_collection: I,
        source: &str,
        frame_id: i32,
    ) -> OscBundle
    where
        E: TuioProfile,
        I: IntoIterator<Item = E>,
    {
        let mut set_messages = vec![];
        let mut session_ids: Vec<OscType> = vec![];
        for profile in profile_collection.into_iter() {
            let session_id = profile.session_id();
            session_ids.push(OscType::Int(session_id));
            set_messages.push(profile.into());
        }

        let alive_message = OscPacket::Message(OscMessage {
            addr: E::address(),
            args: vec![OscType::String("alive".into())]
                .into_iter()
                .chain(session_ids)
                .collect(),
        });
        let mut preamble = vec![alive_message];

        let source_message = OscPacket::Message(OscMessage {
            addr: E::address(),
            args: vec![
                OscType::String("source".into()),
                OscType::String(source.into()),
            ],
        });
        preamble.push(source_message);

        let frame_message = OscPacket::Message(OscMessage {
            addr: E::address(),
            args: vec![OscType::String("fseq".into()), OscType::Int(frame_id)],
        });

        OscBundle {
            timetag: OscTime::try_from(SystemTime::now()).unwrap(),
            content: preamble
                .into_iter()
                .chain(set_messages)
                .chain(iter::once(frame_message))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;
    use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};

    use crate::{
        core::{Position, Velocity},
        tuio11::{Cursor, Object, bundle::EntityType},
    };

    use super::*;

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn now_timetag() -> OscTime {
        OscTime::try_from(std::time::SystemTime::now()).unwrap()
    }

    /// Build a minimal but valid TUIO 1.1 cursor bundle by hand.
    fn make_cursor_bundle(
        session_id: i32,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        accel: f32,
        fseq: i32,
        source: Option<&str>,
    ) -> OscBundle {
        let mut content = Vec::new();

        // optional source message
        if let Some(src) = source {
            content.push(OscPacket::Message(OscMessage {
                addr: "/tuio/2Dcur".to_string(),
                args: vec![
                    OscType::String("source".to_string()),
                    OscType::String(src.to_string()),
                ],
            }));
        }

        // alive message
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![
                OscType::String("alive".to_string()),
                OscType::Int(session_id),
            ],
        }));

        // set message  (7 args)
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

        // fseq message
        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dcur".to_string(),
            args: vec![OscType::String("fseq".to_string()), OscType::Int(fseq)],
        }));

        OscBundle {
            timetag: now_timetag(),
            content,
        }
    }

    /// Build a minimal but valid TUIO 1.1 object bundle by hand (11 args per set).
    #[allow(clippy::too_many_arguments)]
    fn make_object_bundle(
        session_id: i32,
        class_id: i32,
        x: f32,
        y: f32,
        angle: f32,
        vx: f32,
        vy: f32,
        rot_speed: f32,
        accel: f32,
        rot_accel: f32,
        fseq: i32,
        source: Option<&str>,
    ) -> OscBundle {
        let mut content = Vec::new();

        if let Some(src) = source {
            content.push(OscPacket::Message(OscMessage {
                addr: "/tuio/2Dobj".to_string(),
                args: vec![
                    OscType::String("source".to_string()),
                    OscType::String(src.to_string()),
                ],
            }));
        }

        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![
                OscType::String("alive".to_string()),
                OscType::Int(session_id),
            ],
        }));

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

        content.push(OscPacket::Message(OscMessage {
            addr: "/tuio/2Dobj".to_string(),
            args: vec![OscType::String("fseq".to_string()), OscType::Int(fseq)],
        }));

        OscBundle {
            timetag: now_timetag(),
            content,
        }
    }

    // ── OscDecoder::decode_bundle ─────────────────────────────────────────────

    #[test]
    fn decode_cursor_bundle_fseq() {
        let bundle = make_cursor_bundle(1, 0.5, 0.5, 0.0, 0.0, 0.0, 42, None);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.fseq(), 42);
    }

    #[test]
    fn decode_cursor_bundle_source() {
        let bundle = make_cursor_bundle(1, 0.5, 0.5, 0.0, 0.0, 0.0, 1, Some("myserver"));
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.source(), &Some("myserver".to_string()));
    }

    #[test]
    fn decode_cursor_bundle_source_none_when_absent() {
        let bundle = make_cursor_bundle(1, 0.5, 0.5, 0.0, 0.0, 0.0, 1, None);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.source(), &None);
    }

    #[test]
    fn decode_cursor_bundle_alive_contains_session_id() {
        let bundle = make_cursor_bundle(7, 0.5, 0.5, 0.0, 0.0, 0.0, 1, None);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert!(decoded.alive().contains(&7));
    }

    #[test]
    fn decode_cursor_bundle_entities_are_cursors() {
        let bundle = make_cursor_bundle(3, 0.2, 0.8, 0.1, 0.2, 0.5, 5, Some("src"));
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        match decoded.tuio_entities() {
            Some(EntityType::Cursor(cursors)) => {
                assert_eq!(cursors.len(), 1);
                let c = &cursors[0];
                assert_eq!(c.session_id(), 3);
                assert_relative_eq!(c.position().x, 0.2);
                assert_relative_eq!(c.position().y, 0.8);
                assert_relative_eq!(c.velocity().x, 0.1);
                assert_relative_eq!(c.velocity().y, 0.2);
                assert_relative_eq!(c.acceleration(), 0.5);
            }
            other => panic!("expected EntityType::Cursor, got {other:?}"),
        }
    }

    #[test]
    fn decode_object_bundle_fseq() {
        let bundle = make_object_bundle(1, 2, 0.3, 0.4, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 99, None);
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        assert_eq!(decoded.fseq(), 99);
    }

    #[test]
    fn decode_object_bundle_entities_are_objects() {
        let bundle = make_object_bundle(
            8,
            3,
            0.2,
            0.5,
            1.57,
            0.3,
            0.4,
            0.2,
            0.5,
            0.1,
            12,
            Some("test"),
        );
        let decoded = OscDecoder::decode_bundle(bundle).unwrap();
        match decoded.tuio_entities() {
            Some(EntityType::Object(objects)) => {
                assert_eq!(objects.len(), 1);
                let o = &objects[0];
                assert_eq!(o.session_id(), 8);
                assert_eq!(o.class_id(), 3);
                assert_relative_eq!(o.position().x, 0.2);
                assert_relative_eq!(o.position().y, 0.5);
                assert_relative_eq!(o.angle(), 1.57);
            }
            other => panic!("expected EntityType::Object, got {other:?}"),
        }
    }

    #[test]
    fn decode_bundle_rejects_unknown_address() {
        let bundle = OscBundle {
            timetag: now_timetag(),
            content: vec![OscPacket::Message(OscMessage {
                addr: "/tuio/unknown".to_string(),
                args: vec![OscType::String("alive".to_string())],
            })],
        };
        assert!(OscDecoder::decode_bundle(bundle).is_err());
    }

    #[test]
    fn decode_bundle_rejects_set_with_wrong_arg_count_for_cursor() {
        // A "/tuio/2Dcur" "set" message with only 4 args (needs 7) should be rejected.
        let bundle = OscBundle {
            timetag: now_timetag(),
            content: vec![
                OscPacket::Message(OscMessage {
                    addr: "/tuio/2Dcur".to_string(),
                    args: vec![OscType::String("alive".to_string())],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/tuio/2Dcur".to_string(),
                    args: vec![
                        OscType::String("set".to_string()),
                        OscType::Int(1),
                        OscType::Float(0.5),
                        OscType::Float(0.5),
                        // missing vx, vy, accel
                    ],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/tuio/2Dcur".to_string(),
                    args: vec![OscType::String("fseq".to_string()), OscType::Int(1)],
                }),
            ],
        };
        assert!(OscDecoder::decode_bundle(bundle).is_err());
    }

    // ── OscEncoder::encode_bundle ─────────────────────────────────────────────

    #[test]
    fn encode_cursor_bundle_fseq() {
        let cursors = vec![Cursor::new(
            5,
            Position::new(0.2, 0.5),
            Velocity::new(2.5, 3.1),
            0.5,
        )];
        let encoded = OscEncoder::encode_bundle(cursors, "test", 7);
        // The fseq message is the last content item. Decode it to verify.
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        assert_eq!(decoded.fseq(), 7);
    }

    #[test]
    fn encode_cursor_bundle_source() {
        let cursors = vec![Cursor::new(
            5,
            Position::new(0.2, 0.5),
            Velocity::new(2.5, 3.1),
            0.5,
        )];
        let encoded = OscEncoder::encode_bundle(cursors, "mysource", 1);
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        assert_eq!(decoded.source(), &Some("mysource".to_string()));
    }

    #[test]
    fn encode_cursor_bundle_alive_contains_session_ids() {
        let cursors = vec![
            Cursor::new(5, Position::new(0.2, 0.5), Velocity::new(2.5, 3.1), 0.5),
            Cursor::new(6, Position::new(0.3, 0.6), Velocity::new(1.0, 0.5), 0.2),
        ];
        let encoded = OscEncoder::encode_bundle(cursors, "test", 1);
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        assert!(decoded.alive().contains(&5));
        assert!(decoded.alive().contains(&6));
    }

    #[test]
    fn encode_cursor_bundle_entities_round_trip() {
        let cursors = vec![
            Cursor::new(5, Position::new(0.2, 0.5), Velocity::new(2.5, 3.1), 0.5),
            Cursor::new(6, Position::new(0.3, 0.6), Velocity::new(1.0, 0.5), 0.2),
        ];
        let encoded = OscEncoder::encode_bundle(cursors, "test", 5);
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        match decoded.tuio_entities() {
            Some(EntityType::Cursor(decoded_cursors)) => {
                assert_eq!(decoded_cursors.len(), 2);
            }
            other => panic!("expected EntityType::Cursor, got {other:?}"),
        }
    }

    #[test]
    fn encode_object_bundle_round_trip_fseq() {
        let objects = vec![
            Object::new(
                8,
                3,
                Position::new(0.2, 0.5),
                Velocity::new(2.5, 3.1),
                0.5,
                1.57,
                0.2,
                0.1,
            ),
            Object::new(
                12,
                27,
                Position::new(0.1, 0.9),
                Velocity::new(0.5, 0.8),
                0.3,
                2.34,
                0.1,
                0.05,
            ),
        ];
        let encoded = OscEncoder::encode_bundle(objects, "test", 12);
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        assert_eq!(decoded.fseq(), 12);
    }

    #[test]
    fn encode_object_bundle_round_trip_source() {
        let objects = vec![Object::new(
            1,
            0,
            Position::new(0.0, 0.0),
            Velocity::new(0.0, 0.0),
            0.0,
            0.0,
            0.0,
            0.0,
        )];
        let encoded = OscEncoder::encode_bundle(objects, "objsource", 1);
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        assert_eq!(decoded.source(), &Some("objsource".to_string()));
    }

    #[test]
    fn encode_object_bundle_entities_count() {
        let objects = vec![
            Object::new(
                8,
                3,
                Position::new(0.2, 0.5),
                Velocity::new(2.5, 3.1),
                0.5,
                1.57,
                0.2,
                0.1,
            ),
            Object::new(
                12,
                27,
                Position::new(0.1, 0.9),
                Velocity::new(0.5, 0.8),
                0.3,
                1.45,
                0.1,
                0.05,
            ),
        ];
        let encoded = OscEncoder::encode_bundle(objects, "test", 12);
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        match decoded.tuio_entities() {
            Some(EntityType::Object(objects)) => {
                assert_eq!(objects.len(), 2);
            }
            other => panic!("expected EntityType::Object, got {other:?}"),
        }
    }

    #[test]
    fn encode_empty_cursor_bundle_round_trip() {
        // Encoding an empty collection should produce a valid bundle with no entities.
        let cursors: Vec<Cursor> = vec![];
        let encoded = OscEncoder::encode_bundle(cursors, "empty", 0);
        // decode_bundle should still succeed without error.
        let decoded = OscDecoder::decode_bundle(encoded).unwrap();
        assert_eq!(decoded.fseq(), 0);
        assert_eq!(decoded.tuio_entities(), &None);
    }

    #[test]
    fn encode_bundle_result_is_osc_bundle() {
        let cursors = vec![Cursor::new(
            1,
            Position::new(0.0, 0.0),
            Velocity::new(0.0, 0.0),
            0.0,
        )];
        let bundle = OscEncoder::encode_bundle(cursors, "src", 1);
        // The returned value is an OscBundle (all content items should be messages).
        for item in &bundle.content {
            assert!(
                matches!(item, OscPacket::Message(_)),
                "expected all content items to be messages"
            );
        }
    }
}
