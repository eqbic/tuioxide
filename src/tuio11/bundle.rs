use std::collections::HashSet;

use log::warn;
use rosc::{OscMessage, OscType};

use crate::{
    core::TuioError,
    tuio11::{Blob, Cursor, Object},
};

#[derive(Debug, Clone, Default, Copy)]
pub(crate) enum TuioBundleType {
    Cursor,
    Object,
    Blob,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EntityType {
    Cursor(Vec<Cursor>),
    Object(Vec<Object>),
    Blob(Vec<Blob>),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TuioBundle {
    tuio_type: TuioBundleType,
    source: Option<String>,
    alive: HashSet<i32>,
    set: Option<EntityType>,
    fseq: i32,
}

impl TuioBundle {
    pub(crate) fn profile_type(&self) -> TuioBundleType {
        self.tuio_type
    }

    pub(crate) fn source(&self) -> &Option<String> {
        &self.source
    }

    pub(crate) fn alive(&self) -> &HashSet<i32> {
        &self.alive
    }

    pub(crate) fn tuio_entities(&self) -> &Option<EntityType> {
        &self.set
    }

    pub(crate) fn fseq(&self) -> i32 {
        self.fseq
    }

    pub(crate) fn set_type(&mut self, tuio_type: TuioBundleType) {
        self.tuio_type = tuio_type
    }

    pub(crate) fn set_source(&mut self, message: &OscMessage) {
        self.source = message.args.get(1).and_then(|arg| arg.clone().string());
    }

    pub(crate) fn set_alive(&mut self, message: &OscMessage) {
        self.alive = message
            .args
            .iter()
            .skip(1)
            .filter_map(|e| e.clone().int())
            .collect();
    }

    pub(crate) fn set_set(&mut self, message: &OscMessage) -> Result<(), TuioError> {
        match &self.tuio_type {
            TuioBundleType::Cursor => {
                if let EntityType::Cursor(set) =
                    self.set.get_or_insert(EntityType::Cursor(Vec::new()))
                {
                    if message.args.len() != 7 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let cursor = Cursor::try_from(message)?;
                    set.push(cursor);
                }
            }
            TuioBundleType::Object => {
                if let EntityType::Object(set) =
                    self.set.get_or_insert(EntityType::Object(Vec::new()))
                {
                    if message.args.len() != 11 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let object = Object::try_from(message)?;
                    set.push(object);
                }
            }
            TuioBundleType::Blob => {
                if let EntityType::Blob(set) = self.set.get_or_insert(EntityType::Blob(Vec::new()))
                {
                    if message.args.len() != 13 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let blob = Blob::try_from(message)?;
                    set.push(blob);
                }
            }
            TuioBundleType::Unknown => {
                warn!("Unknown Tuio Bundle Type")
            }
        }
        Ok(())
    }

    pub(crate) fn set_fseq(&mut self, message: &OscMessage) -> Result<(), TuioError> {
        if let Some(OscType::Int(fseq)) = message.args.get(1) {
            self.fseq = *fseq;
            Ok(())
        } else {
            Err(TuioError::MissingArguments(message.clone()))
        }
    }
}
