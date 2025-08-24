use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};

use std::{iter, time::SystemTime};

use crate::tuio11::profile::Profile;

use crate::{
    common::errors::TuioError,
    tuio11::{blob::Blob, cursor::Cursor, object::Object},
};

#[derive(Debug, Clone, Default)]
pub enum TuioBundleType {
    Cursor,
    Object,
    Blob,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Set {
    Cursor(Vec<Cursor>),
    Object(Vec<Object>),
    Blob(Vec<Blob>),
}

#[derive(Debug, Clone, Default)]
pub struct TuioBundle {
    tuio_type: TuioBundleType,
    souce: Option<String>,
    alive: Vec<i32>,
    set: Option<Set>,
    fseq: i32,
}

impl TuioBundle {
    pub fn source(&self) -> &Option<String> {
        &self.souce
    }

    pub fn alive(&self) -> &Vec<i32> {
        &self.alive
    }

    pub fn tuio_entities(&self) -> &Option<Set> {
        &self.set
    }

    pub fn fseq(&self) -> i32 {
        self.fseq
    }

    fn set_source(&mut self, message: &OscMessage) {
        self.souce = message.args.get(1).and_then(|arg| arg.clone().string());
    }

    fn set_alive(&mut self, message: &OscMessage) {
        self.alive = message
            .args
            .iter()
            .skip(1)
            .filter_map(|e| e.clone().int())
            .collect();
    }

    fn set_set(&mut self, message: &OscMessage) -> Result<(), TuioError> {
        match &self.tuio_type {
            TuioBundleType::Cursor => {
                if let Set::Cursor(set) = self.set.get_or_insert(Set::Cursor(Vec::new())) {
                    if message.args.len() != 7 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let cursor = Cursor::try_from(message)?;
                    set.push(cursor);
                }
            }
            TuioBundleType::Object => {
                if let Set::Object(set) = self.set.get_or_insert(Set::Object(Vec::new())) {
                    if message.args.len() != 11 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let object = Object::try_from(message)?;
                    set.push(object);
                }
            }
            TuioBundleType::Blob => {
                if let Set::Blob(set) = self.set.get_or_insert(Set::Object(Vec::new())) {
                    if message.args.len() != 13 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let blob = Blob::try_from(message)?;
                    set.push(blob);
                }
            }
            TuioBundleType::Unknown => {}
        }
        Ok(())
    }

    fn set_fseq(&mut self, message: &OscMessage) -> Result<(), TuioError> {
        if let Some(OscType::Int(fseq)) = message.args.get(1) {
            self.fseq = *fseq;
            Ok(())
        } else {
            return Err(TuioError::MissingArguments(message.clone()).into());
        }
    }
}

pub struct OscDecoder;

impl OscDecoder {
    pub fn decode_bundle(bundle: OscBundle) -> Result<TuioBundle, TuioError> {
        let mut tuio_bundle = TuioBundle::default();
        for packet in &bundle.content {
            if let OscPacket::Message(message) = packet {
                tuio_bundle.tuio_type = match message.addr.as_str() {
                    "/tuio/2Dcur" => TuioBundleType::Cursor,
                    "/tuio/2Dobj" => TuioBundleType::Object,
                    "/tuio/2Dblb" => TuioBundleType::Blob,
                    _ => return Err(TuioError::UnknownAddress(message.clone())),
                };

                match message.args.first() {
                    Some(OscType::String(arg)) => match arg.as_str() {
                        "source" => tuio_bundle.set_source(&message),
                        "alive" => tuio_bundle.set_alive(&message),
                        "set" => tuio_bundle.set_set(&message)?,
                        "fseq" => tuio_bundle.set_fseq(&message)?,
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

pub struct OscEncoder;

impl OscEncoder {
    pub fn encode_bundle<'a, T, I>(
        profile_collection: I,
        source: Option<&str>,
        frame_id: i32,
    ) -> OscBundle
    where
        T: Profile<'a>,
        I: IntoIterator<Item = T>,
    {
        let mut set_messages = vec![];
        let mut session_ids: Vec<OscType> = vec![];
        for profile in profile_collection.into_iter() {
            let session_id = profile.session_id();
            session_ids.push(OscType::Int(session_id));
            set_messages.push(profile.into());
        }

        let alive_message = OscPacket::Message(OscMessage {
            addr: T::address(),
            args: vec![OscType::String("alive".into())]
                .into_iter()
                .chain(session_ids.into_iter())
                .collect(),
        });
        let mut preamble = vec![alive_message];

        if let Some(source) = source {
            let source_message = OscPacket::Message(OscMessage {
                addr: T::address(),
                args: vec![
                    OscType::String("source".into()),
                    OscType::String(source.into()),
                ],
            });
            preamble.push(source_message);
        }

        let frame_message = OscPacket::Message(OscMessage {
            addr: T::address(),
            args: vec![OscType::String("fseq".into()), OscType::Int(frame_id)],
        });

        OscBundle {
            timetag: OscTime::try_from(SystemTime::now()).unwrap(),
            content: preamble
                .into_iter()
                .chain(set_messages.into_iter())
                .chain(iter::once(frame_message))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use euclid::default::{Point2D, Vector2D};

    use crate::tuio11::{cursor::Cursor, object::Object};

    use super::*;

    #[test]
    fn test_encode() {
        let source = "test";

        let cursors = vec![
            Cursor::new(5, Point2D::new(0.2, 0.5), Vector2D::new(2.5, 3.1), 0.5),
            Cursor::new(6, Point2D::new(0.2, 0.5), Vector2D::new(2.5, 3.1), 0.5),
        ];
        let objects = vec![
            Object::new(
                8,
                3,
                Point2D::new(0.2, 0.5),
                2.5,
                Vector2D::new(2.5, 3.1),
                5.2,
                1.4,
                3.5,
            ),
            Object::new(
                12,
                27,
                Point2D::new(0.2, 0.5),
                2.5,
                Vector2D::new(2.5, 3.1),
                5.2,
                1.4,
                3.5,
            ),
        ];
        // let blobs = vec![];

        let cursor_bundle = OscEncoder::encode_bundle(cursors.clone(), Some(source), 5);
        let object_bundle = OscEncoder::encode_bundle(objects.clone(), Some(source), 12);
        // let blob_bundle = OscEncoder::encode_blob_bundle(&blobs, source, 0);
        let tuio_cursor_bundle =
            OscDecoder::decode_bundle(cursor_bundle).expect("Could not decode cursor bundle");
        let tuio_object_bundle =
            OscDecoder::decode_bundle(object_bundle).expect("Could not decode object bundle");

        assert_eq!(tuio_cursor_bundle.fseq(), 5);
        assert_eq!(tuio_cursor_bundle.alive(), &vec![5, 6]);
        assert_eq!(tuio_cursor_bundle.source(), &Some("test".into()));
        assert_eq!(
            tuio_cursor_bundle.tuio_entities(),
            &Some(Set::Cursor(cursors))
        );

        assert_eq!(tuio_object_bundle.fseq(), 12);
        assert_eq!(tuio_object_bundle.alive(), &vec![8, 12]);
        assert_eq!(tuio_object_bundle.source(), &Some("test".into()));
        assert_eq!(
            tuio_object_bundle.tuio_entities(),
            &Some(Set::Object(objects))
        );
    }
}
