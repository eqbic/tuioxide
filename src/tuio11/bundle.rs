use std::collections::HashSet;

use rosc::{OscMessage, OscType};

use crate::{
    common::errors::TuioError,
    tuio11::{blob::BlobProfile, cursor::CursorProfile, object::ObjectProfile},
};

#[derive(Debug, Clone, Default, Copy)]
pub enum TuioBundleType {
    Cursor,
    Object,
    Blob,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Cursor(Vec<CursorProfile>),
    Object(Vec<ObjectProfile>),
    Blob(Vec<BlobProfile>),
}

#[derive(Debug, Clone, Default)]
pub struct TuioBundle {
    tuio_type: TuioBundleType,
    source: Option<String>,
    alive: HashSet<i32>,
    set: Option<EntityType>,
    fseq: i32,
}

impl TuioBundle {
    pub fn profile_type(&self) -> TuioBundleType {
        self.tuio_type
    }

    pub fn source(&self) -> &Option<String> {
        &self.source
    }

    pub fn alive(&self) -> &HashSet<i32> {
        &self.alive
    }

    pub fn tuio_entities(&self) -> &Option<EntityType> {
        &self.set
    }

    pub fn fseq(&self) -> i32 {
        self.fseq
    }

    pub fn set_type(&mut self, tuio_type: TuioBundleType) {
        self.tuio_type = tuio_type
    }

    pub fn set_source(&mut self, message: &OscMessage) {
        self.source = message.args.get(1).and_then(|arg| arg.clone().string());
    }

    pub fn set_alive(&mut self, message: &OscMessage) {
        self.alive = message
            .args
            .iter()
            .skip(1)
            .filter_map(|e| e.clone().int())
            .collect();
    }

    pub fn set_set(&mut self, message: &OscMessage) -> Result<(), TuioError> {
        match &self.tuio_type {
            TuioBundleType::Cursor => {
                if let EntityType::Cursor(set) =
                    self.set.get_or_insert(EntityType::Cursor(Vec::new()))
                {
                    if message.args.len() != 7 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let cursor = CursorProfile::try_from(message)?;
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
                    let object = ObjectProfile::try_from(message)?;
                    set.push(object);
                }
            }
            TuioBundleType::Blob => {
                if let EntityType::Blob(set) =
                    self.set.get_or_insert(EntityType::Object(Vec::new()))
                {
                    if message.args.len() != 13 {
                        return Err(TuioError::MissingArguments(message.clone()));
                    }
                    let blob = BlobProfile::try_from(message)?;
                    set.push(blob);
                }
            }
            TuioBundleType::Unknown => {}
        }
        Ok(())
    }

    pub fn set_fseq(&mut self, message: &OscMessage) -> Result<(), TuioError> {
        if let Some(OscType::Int(fseq)) = message.args.get(1) {
            self.fseq = *fseq;
            Ok(())
        } else {
            Err(TuioError::MissingArguments(message.clone()))
        }
    }
}
