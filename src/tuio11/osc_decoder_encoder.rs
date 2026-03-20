use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};

use std::{iter, time::SystemTime};

use crate::core::TuioEntity;

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
        source: Option<&str>,
        frame_id: i32,
    ) -> OscBundle
    where
        E: TuioEntity,
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

        if let Some(source) = source {
            let source_message = OscPacket::Message(OscMessage {
                addr: E::address(),
                args: vec![
                    OscType::String("source".into()),
                    OscType::String(source.into()),
                ],
            });
            preamble.push(source_message);
        }

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

// #[cfg(test)]
// mod tests {

//     use crate::{
//         core::{Position, Velocity},
//         tuio11::{Cursor, bundle::EntityType, cursor::CursorProfile, object::ObjectProfile},
//     };

//     use super::*;

//     #[test]
//     fn test_encode() {
//         let source = "test";

//         let cursors = vec![
//             Cursor::new(&TuioTime::5, Position::new(0.2, 0.5), Velocity::new(2.5, 3.1), 0.5),
//             Cursor::new(6, Position::new(0.2, 0.5), Velocity::new(2.5, 3.1), 0.5),
//         ];
//         let objects = vec![
//             ObjectProfile::new(
//                 8,
//                 3,
//                 Position::new(0.2, 0.5),
//                 2.5,
//                 Velocity::new(2.5, 3.1),
//                 5.2,
//                 1.4,
//                 3.5,
//             ),
//             ObjectProfile::new(
//                 12,
//                 27,
//                 Position::new(0.2, 0.5),
//                 2.5,
//                 Velocity::new(2.5, 3.1),
//                 5.2,
//                 1.4,
//                 3.5,
//             ),
//         ];
//         // let blobs = vec![];

//         let cursor_bundle = OscEncoder::encode_bundle(cursors.clone(), Some(source), 5);
//         let object_bundle = OscEncoder::encode_bundle(objects.clone(), Some(source), 12);
//         // let blob_bundle = OscEncoder::encode_blob_bundle(&blobs, source, 0);
//         let tuio_cursor_bundle =
//             OscDecoder::decode_bundle(cursor_bundle).expect("Could not decode cursor bundle");
//         let tuio_object_bundle =
//             OscDecoder::decode_bundle(object_bundle).expect("Could not decode object bundle");

//         assert_eq!(tuio_cursor_bundle.fseq(), 5);
//         // assert_eq!(tuio_cursor_bundle.alive(), &vec![5, 6]);
//         assert_eq!(tuio_cursor_bundle.source(), &Some("test".into()));
//         assert_eq!(
//             tuio_cursor_bundle.tuio_entities(),
//             &Some(EntityType::Cursor(cursors))
//         );

//         assert_eq!(tuio_object_bundle.fseq(), 12);
//         // assert_eq!(tuio_object_bundle.alive(), &vec![8, 12]);
//         assert_eq!(tuio_object_bundle.source(), &Some("test".into()));
//         assert_eq!(
//             tuio_object_bundle.tuio_entities(),
//             &Some(EntityType::Object(objects))
//         );
//     }
// }
