use std::{
    io,
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::{
    core::osc_receiver::{OscReceiver, UdpOscReceiver},
    tuio11::{TuioEvents, processor::Processor},
};

/// A high-level TUIO 1.1 client that receives OSC packets and processes them
/// into typed TUIO events.
///
/// `Client` accepts any [`OscReceiver`] implementation, making it usable with
/// both UDP and WebSocket transports. The most common usage is with the provided
/// [`UdpOscReceiver`], available via [`Client::default()`].
///
/// # Example
///
/// ```no_run
/// use std::net::Ipv4Addr;
/// use tuioxide::core::osc_receiver::UdpOscReceiver;
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
pub struct Client {
    receiver: Box<dyn OscReceiver>,
    processor: Processor,
}

impl Client {
    /// Creates a new `Client` with the given [`OscReceiver`].
    ///
    /// The processor is initialised with an empty state and will begin tracking
    /// TUIO entities as packets are received via [`Client::update`].
    pub fn new(receiver: impl OscReceiver + 'static) -> Self {
        Self {
            receiver: Box::new(receiver),
            processor: Processor::default(),
        }
    }

    /// Starts a background thread that continuously receives and processes OSC
    /// packets, returning a [`Receiver`] through which [`TuioEvents`] can be
    /// consumed.
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
    /// use tuioxide::tuio11::Client;
    ///
    /// let receiver = UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
    /// let rx = Client::spawn(receiver);
    ///
    /// loop {
    ///     for events in rx.try_iter() {
    ///         for event in events.cursor_events {
    ///             println!("{event:?}");
    ///         }
    ///     }
    /// }
    /// ```
    pub fn spawn(receiver: impl OscReceiver + 'static) -> Receiver<TuioEvents> {
        let (tx, rx) = mpsc::channel();
        let mut client = Client::new(receiver);
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

impl Default for Client {
    /// Creates a default `Client` backed by a [`UdpOscReceiver`] bound to
    /// `127.0.0.1:3333`, which is the standard TUIO UDP port.
    fn default() -> Self {
        Self::new(UdpOscReceiver::default())
    }
}
