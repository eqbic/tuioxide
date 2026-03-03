use std::io;

use crate::{
    core::osc_receiver::{OscReceiver, UdpOscReceiver},
    tuio11::{TuioEvents, processor::Processor},
};

/// A high-level TUIO 1.1 client that receives OSC packets and processes them
/// into typed TUIO events.
///
/// `Client` is generic over any [`OscReceiver`] implementation, making it usable
/// with both UDP and WebSocket transports. The most common usage is with the
/// provided [`UdpOscReceiver`], available via [`Client::default()`].
///
/// # Example
///
/// ```no_run
/// use std::net::Ipv4Addr;
/// use tuioxide::core::osc_receiver::{OscReceiver, UdpOscReceiver};
/// use tuioxide::tuio11::Client;
///
/// let receiver = UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
/// let mut client = Client::new(receiver);
///
/// loop {
///     if let Ok(events) = client.update() {
///         for event in events.cursor_events {
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
    /// Creates a new `Client` with the given [`OscReceiver`].
    ///
    /// The processor is initialised with an empty state and will begin tracking
    /// TUIO entities as packets are received via [`Client::update`].
    pub fn new(receiver: T) -> Self {
        Self {
            receiver,
            processor: Processor::default(),
        }
    }

    /// Blocks until one OSC packet is received, then processes it and returns
    /// the resulting TUIO 1.1 events.
    ///
    /// Each call to `update` corresponds to one OSC bundle received from the
    /// TUIO source. The returned [`TuioEvents`] contains separate lists for
    /// cursor, object, and blob events that occurred in that bundle.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the underlying receiver fails to read a packet,
    /// or if the received packet does not contain a valid TUIO bundle.
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
    /// `127.0.0.1:3333`, which is the standard TUIO UDP port.
    fn default() -> Self {
        Self::new(UdpOscReceiver::default())
    }
}
