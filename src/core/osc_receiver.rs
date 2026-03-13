use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
};

use log::{debug, info};
use rosc::{OscPacket, decoder::MTU};

/// A transport-agnostic interface for receiving raw OSC packets.
///
/// Implementors bind to a network address and block until an [`OscPacket`] is
/// available, returning it to the caller. Two implementations are provided out
/// of the box:
///
/// - [`UdpOscReceiver`] — always available, receives OSC over UDP.
/// - [`websocket::WebsocketOscReceiver`] — available with the `websocket` feature,
///   receives OSC over a WebSocket connection.
///
/// You can box any implementation and pass it to [`tuio11::Client`](crate::tuio11::Client)
/// or [`tuio20::Client`](crate::tuio20::Client) to use a custom transport.
pub trait OscReceiver: Send {
    /// Blocks until the next OSC packet is received and returns it.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if reading from the underlying transport fails.
    fn recv(&mut self) -> Result<OscPacket, io::Error>;
}

/// An [`OscReceiver`] that reads OSC packets from a UDP socket.
///
/// Binds a UDP socket to the specified address and port on construction and
/// reads incoming datagrams on each call to [`recv`](OscReceiver::recv).
///
/// # Default
///
/// The [`Default`] implementation binds to `127.0.0.1:3333`, which is the
/// conventional TUIO port.
///
/// # Example
///
/// ```no_run
/// use std::net::Ipv4Addr;
/// use tuioxide::client::osc_receiver::{OscReceiver, UdpOscReceiver};
///
/// let mut receiver = UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
/// let packet = receiver.recv().unwrap();
/// ```
#[derive(Debug)]
pub struct UdpOscReceiver {
    socket: UdpSocket,
    buffer: [u8; MTU],
}

impl UdpOscReceiver {
    /// Binds a UDP socket to `remote:port` and returns a new [`UdpOscReceiver`].
    ///
    /// # Panics
    ///
    /// Panics if the socket cannot be bound to the given address and port.
    fn new(remote: Ipv4Addr, port: u16) -> Self {
        let socket = match UdpSocket::bind(SocketAddrV4::new(remote, port)) {
            Ok(socket) => {
                info!("Created UDP socket for {remote}:{port}");
                socket
            }
            Err(error) => panic!("Could not bind to socket: {}", error),
        };

        Self {
            socket,
            buffer: [0u8; MTU],
        }
    }
}

impl OscReceiver for UdpOscReceiver {
    /// Blocks until a UDP datagram arrives, decodes it as an OSC packet, and returns it.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the underlying [`UdpSocket::recv`] call fails.
    ///
    /// # Panics
    ///
    /// Panics if the received datagram cannot be decoded as a valid OSC packet.
    fn recv(&mut self) -> Result<OscPacket, io::Error> {
        let size = self.socket.recv(&mut self.buffer)?;
        let (_, packet) = rosc::decoder::decode_udp(&self.buffer[..size]).unwrap();
        debug!("{:?}", packet);
        Ok(packet)
    }
}

impl Default for UdpOscReceiver {
    /// Returns a [`UdpOscReceiver`] bound to `127.0.0.1:3333`.
    fn default() -> Self {
        Self::new(Ipv4Addr::LOCALHOST, 3333)
    }
}

/// WebSocket-based OSC transport. Available only with the `websocket` feature flag.
#[cfg(feature = "websocket")]
pub mod websocket {
    use log::{info, warn};
    use rosc::OscPacket;
    use std::{
        io,
        net::{Ipv4Addr, TcpStream},
        thread::sleep,
        time::Duration,
    };
    use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

    use crate::core::osc_receiver::OscReceiver;

    /// An [`OscReceiver`] that reads OSC packets from a WebSocket connection.
    ///
    /// On construction the receiver attempts to connect to the given address and
    /// port, retrying indefinitely with a 2-second delay between attempts until
    /// the connection succeeds.
    ///
    /// Binary WebSocket frames are decoded as OSC packets. Non-binary frames
    /// (e.g. text or ping/pong) are treated as an error.
    ///
    /// # Default
    ///
    /// The [`Default`] implementation connects to `ws://127.0.0.1:3333`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::net::Ipv4Addr;
    /// use tuioxide::client::osc_receiver::OscReceiver;
    /// use tuioxide::client::osc_receiver::websocket::WebsocketOscReceiver;
    ///
    /// let mut receiver = WebsocketOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
    /// let packet = receiver.recv().unwrap();
    /// ```
    #[derive(Debug)]
    pub struct WebsocketOscReceiver {
        socket: WebSocket<MaybeTlsStream<TcpStream>>,
    }

    impl WebsocketOscReceiver {
        /// Connects to `ws://remote:port` (retrying until successful) and returns
        /// a new [`WebsocketOscReceiver`].
        fn new(remote: Ipv4Addr, port: u16) -> Self {
            let socket = WebsocketOscReceiver::connect_with_retry(remote, port);
            Self { socket }
        }

        /// Attempts to open a WebSocket connection to `ws://remote:port`, retrying
        /// every 2 seconds until the connection succeeds.
        fn connect_with_retry(remote: Ipv4Addr, port: u16) -> WebSocket<MaybeTlsStream<TcpStream>> {
            let uri = format!("ws://{remote}:{port}");
            loop {
                match connect(uri.as_str()) {
                    Ok((socket, _)) => {
                        info!("Successfully connected to {uri}");
                        return socket;
                    }
                    Err(e) => {
                        warn!("Could not connect to {uri}: {e}. Try again...");
                        sleep(Duration::from_secs(2));
                    }
                }
            }
        }
    }

    impl OscReceiver for WebsocketOscReceiver {
        /// Blocks until the next WebSocket message arrives, decodes it as an OSC
        /// packet, and returns it.
        ///
        /// # Errors
        ///
        /// Returns an [`io::Error`] with [`io::ErrorKind::InvalidData`] if the
        /// received frame is not a binary message.
        ///
        /// # Panics
        ///
        /// Panics if reading from the WebSocket fails or if the binary payload
        /// cannot be decoded as a valid OSC packet.
        fn recv(&mut self) -> Result<OscPacket, io::Error> {
            let message = self.socket.read().unwrap();
            if let Message::Binary(data) = message {
                let (_, packet) = rosc::decoder::decode_udp(&data).unwrap();
                Ok(packet)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))
            }
        }
    }

    impl Default for WebsocketOscReceiver {
        /// Returns a [`WebsocketOscReceiver`] connected to `ws://127.0.0.1:3333`.
        fn default() -> Self {
            Self::new(Ipv4Addr::LOCALHOST, 3333)
        }
    }
}
