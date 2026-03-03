# tuioxide

A Rust implementation of the [TUIO](https://tuio.org) protocol by Martin Kaltenbrunner, supporting both [TUIO 1.1](https://tuio.org/?specification) and [TUIO 2.0](https://www.tuio.org/?tuio20). Built on top of [rosc](https://crates.io/crates/rosc) for OSC packet encoding and decoding.

TUIO is an open framework that defines a common protocol and API for tangible multitouch surfaces. It allows applications to receive touch, tangible object, and gesture data from interactive surfaces and trackers.

## Features

- **TUIO 1.1** — Cursors (`/tuio/2Dcur`), Objects (`/tuio/2Dobj`), Blobs (`/tuio/2Dblb`)
- **TUIO 2.0** — Pointers (`/tuio2/ptr`), Tokens (`/tuio2/tok`), Bounds (`/tuio2/bnd`), Symbols (`/tuio2/sym`)
- **UDP** transport (always available)
- **WebSocket** transport (optional feature flag)
- **Event-driven API** — receive `Add`, `Update`, and `Remove` events for each entity type
- **Generic client** — plug in any transport that implements `OscReceiver`

## Roadmap
- [x] Tuio 1.1
- [x] Tuio 2.0
- [x] Client (Udp/WebSocket)
- [ ] Server (Udp/WebSocket)

## Installation

Add tuioxide to your `Cargo.toml`:

```toml
[dependencies]
tuioxide = { version = "0.2.0" }
```

To enable WebSocket support, enable the `websocket` feature:

```toml
[dependencies]
tuioxide = { version = "0.2.0", features = ["websocket"] }
```

## Usage

All clients listen in a blocking loop and return a batch of events each time `update()` is called. By default, the client connects to `127.0.0.1:3333`.

### TUIO 1.1 over UDP

```rust
use tuioxide::{
    tuio11::Client,
    tuio11::event::CursorEvent,
};

fn main() {
    let mut client = Client::default(); // listens on 127.0.0.1:3333
    loop {
        let events = client.update().unwrap();
        for event in events.cursor_events {
            match event {
                CursorEvent::Add(cursor) => println!(
                    "New cursor [{}] at {:?}",
                    cursor.session_id(),
                    cursor.position()
                ),
                CursorEvent::Update(cursor) => println!(
                    "Update cursor [{}] -> {:?}",
                    cursor.session_id(),
                    cursor.position()
                ),
                CursorEvent::Remove(cursor) => {
                    println!("Remove cursor [{}]", cursor.session_id())
                }
            }
        }
    }
}
```

### TUIO 2.0 over UDP

```rust
use tuioxide::{
    tuio20::Client,
    tuio20::{PointerEvent, TokenEvent},
};

fn main() {
    let mut client = Client::default(); // listens on 127.0.0.1:3333
    loop {
        let events = client.update().unwrap();
        for event in events.pointer_events {
            match event {
                PointerEvent::Add(pointer) => println!(
                    "New pointer [{}] at {:?}",
                    pointer.session_id(),
                    pointer.position()
                ),
                PointerEvent::Update(pointer) => println!(
                    "Update pointer [{}] -> {:?}",
                    pointer.session_id(),
                    pointer.position()
                ),
                PointerEvent::Remove(pointer) => {
                    println!("Remove pointer [{}]", pointer.session_id())
                }
            }
        }
    }
}
```

### WebSocket transport

Enable the `websocket` feature and pass a `WebsocketOscReceiver` to the client:

```rust
use tuioxide::{
    core::{WebsocketOscReceiver, tuio11::Client},
    tuio11::event::CursorEvent,
};

fn main() {
    // Connects to ws://127.0.0.1:3333 with automatic retry
    let mut client = Client::new(WebsocketOscReceiver::default());
    loop {
        let events = client.update().unwrap();
        // handle events ...
    }
}
```

### Custom transport

Implement the `OscReceiver` trait to use any transport layer:

```rust
use std::{io, net::Ipv4Addr};
use rosc::OscPacket;
use tuioxide::core::osc_receiver::OscReceiver;

struct MyReceiver { /* ... */ }

impl OscReceiver for MyReceiver {
    fn new(remote: Ipv4Addr, port: u16) -> Self { todo!() }
    fn recv(&mut self) -> Result<OscPacket, io::Error> { todo!() }
}
```

Then pass it to any client:

```rust
let mut client = tuioxide::tuio11::Client::new(MyReceiver::new(...));
```

## Examples

All examples connect to `127.0.0.1:3333`. Run them with `cargo run`:

```sh
# TUIO 1.1 over UDP
cargo run --example receive_tuio_11_udp

# TUIO 1.1 over WebSocket
cargo run --example receive_tuio_11_websocket --features websocket

# TUIO 2.0 over UDP
cargo run --example receive_tuio_20_udp

# TUIO 2.0 over WebSocket
cargo run --example receive_tuio_20_websocket --features websocket
```

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- [TUIO specification](https://tuio.org) by Martin Kaltenbrunner
- [rosc](https://crates.io/crates/rosc) — Rust OSC library
