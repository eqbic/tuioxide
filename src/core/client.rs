use std::{
    io,
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::core::{
    osc_receiver::{OscReceiver, UdpOscReceiver},
    processor::TuioProcessor,
};
/// A high-level TUIO client that receives OSC packets and processes them
/// into typed TUIO events.
///
/// `Client` is generic over a [`TuioProcessor`] implementation, which determines
/// the TUIO protocol version and the type of events produced. In practice this
/// type parameter is always set via the type aliases [`tuio11::Client`] and
/// [`tuio20::Client`] — direct use of `Client<P>` is not required.
///
/// `Client` accepts any [`OscReceiver`] implementation, making it usable with
/// both UDP and WebSocket transports. The default configuration uses
/// [`UdpOscReceiver`] bound to `127.0.0.1:3333`.
pub struct Client<P: TuioProcessor> {
    receiver: Box<dyn OscReceiver>,
    processor: P,
}

impl<P: TuioProcessor> Client<P> {
    /// Creates a new `Client` with the given [`OscReceiver`].
    ///
    /// The processor is initialised in its default state with no tracked
    /// entities.
    pub fn new(receiver: impl OscReceiver + 'static) -> Self {
        Self {
            receiver: Box::new(receiver),
            processor: P::default(),
        }
    }

    /// Starts a background thread that continuously receives and processes OSC
    /// packets, returning a [`Receiver`] through which [`P::Events`](TuioProcessor::Events)
    /// can be consumed.
    ///
    /// This is the non-blocking alternative to [`Client::update`]. Rather than
    /// driving the receive loop manually, the caller polls or iterates the returned
    /// [`Receiver`] at its own pace.
    ///
    /// The background thread exits if the underlying receiver encounters an error
    /// (e.g. the socket is closed) or if the [`Receiver`] is dropped.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::net::Ipv4Addr;
    /// use tuioxide::core::osc_receiver::UdpOscReceiver;
    /// use tuioxide::tuio20::Client;
    ///
    /// let receiver = UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
    /// let rx = Client::spawn(receiver);
    ///
    /// loop {
    ///     for events in rx.try_iter() {
    ///         for event in events.pointer_events {
    ///             println!("{event:?}");
    ///         }
    ///     }
    /// }
    /// ```
    pub fn spawn(receiver: impl OscReceiver + 'static) -> Receiver<P::Events>
    where
        P: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let mut client: Client<P> = Client::new(receiver);
        thread::spawn(move || {
            loop {
                let packet = match client.receiver.recv() {
                    Ok(packet) => packet,
                    Err(error) => {
                        log::error!("OSC receiver error: {error}");
                        break;
                    }
                };

                if let Some(events) = client.processor.update(packet)
                    && tx.send(events).is_err()
                {
                    break;
                }
            }
        });
        rx
    }

    /// Blocks until the next OSC packet is received, processes it, and returns
    /// the resulting events.
    ///
    /// Each call corresponds to one TUIO frame. The type of the returned events
    /// depends on the [`TuioProcessor`] in use — [`tuio11::TuioEvents`] for TUIO 1.1
    /// or [`tuio20::TuioEvents`] for TUIO 2.0.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the underlying receiver fails to read a packet,
    /// or if the received packet is not a valid TUIO bundle.
    pub fn update(&mut self) -> Result<P::Events, io::Error> {
        let packet = self.receiver.recv()?;
        self.processor.update(packet).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "No valid Tuio Bundle",
        ))
    }
}

impl<P: TuioProcessor> Default for Client<P> {
    /// Creates a default `Client` backed by a [`UdpOscReceiver`] bound to
    /// `127.0.0.1:3333`.
    fn default() -> Self {
        Self::new(UdpOscReceiver::default())
    }
}
