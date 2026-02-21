use rosc::{OscBundle, OscPacket};

use crate::{core::errors::TuioError, core::tuio20::bundle::TuioBundle};

pub struct OscDecoder;

impl OscDecoder {
    pub fn decode_bundle(bundle: OscBundle) -> Result<TuioBundle, TuioError> {
        let mut tuio_bundle = TuioBundle::default();
        for packet in &bundle.content {
            if let OscPacket::Message(message) = packet {
                match message.addr.as_str() {
                    "/tuio2/frm" => tuio_bundle.set_frame(message),
                    "/tuio2/ptr" => tuio_bundle.add_pointer(message),
                    "/tuio2/tok" => tuio_bundle.add_token(message),
                    "/tuio2/bnd" => tuio_bundle.add_bounds(message),
                    "/tuio2/sym" => tuio_bundle.add_symbol(message),
                    "/tuio2/alv" => tuio_bundle.set_alive(message),
                    _ => return Err(TuioError::UnknownAddress(message.clone())),
                }
            }
        }
        Ok(tuio_bundle)
    }
}
