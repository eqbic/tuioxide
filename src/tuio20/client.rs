use std::io;

use crate::{
    core::osc_receiver::{OscReceiver, UdpOscReceiver},
    tuio20::{TuioEvents, processor::Processor},
};

/// A high-level TUIO 2.0 client that receives OSC bundles and processes them
/// into typed TUIO events.
///
/// `Client` is generic over any [`OscReceiver`] transport (e.g. UDP or WebSocket),
/// allowing it to be used with different network backends. The default configuration
/// uses [`UdpOscReceiver`] bound to `127.0.0.1:3333`.
///
/// # Example
///
/// ```no_run
/// use std::net::Ipv4Addr;
/// use tuioxide::core::osc_receiver::{OscReceiver, UdpOscReceiver};
/// use tuioxide::tuio20::Client;
///
/// let receiver = UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
/// let mut client = Client::new(receiver);
///
/// loop {
///     if let Ok(events) = client.update() {
///         for event in events.pointer_events {
///             println!("{event:?}");
///         }
///     }
/// }
/// ```
pub struct Client<T: OscReceiver> {
    receiver: T,
    processor: Processor,
}

impl<T> Client<T>
where
    T: OscReceiver,
{
    /// Creates a new `Client` using the given `receiver` as its OSC transport.
    ///
    /// The internal [`Processor`] is initialised in its default state with no
    /// tracked entities and a frame counter starting at `-1`.
    pub fn new(receiver: T) -> Self {
        Self {
            receiver,
            processor: Processor::default(),
        }
    }

    /// Blocks until the next OSC packet is received, decodes it, and returns
    /// the resulting [`TuioEvents`].
    ///
    /// Each call corresponds to one TUIO frame. The returned [`TuioEvents`] contains
    /// separate vectors of events for pointers, tokens, bounds, and symbols — each
    /// event indicating whether the entity was added, updated, or removed.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the underlying receiver fails to read a packet,
    /// or if the received packet is not a valid TUIO 2.0 bundle.
    pub fn update(&mut self) -> Result<TuioEvents, io::Error> {
        let packet = self.receiver.recv()?;
        self.processor.update(packet).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "No valid Tuio Bundle",
        ))
    }
}

impl Default for Client<UdpOscReceiver> {
    /// Creates a default `Client` backed by a [`UdpOscReceiver`] bound to
    /// `127.0.0.1:3333`.
    fn default() -> Self {
        Self::new(UdpOscReceiver::default())
    }
}
