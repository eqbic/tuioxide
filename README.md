# tuioxide

# IMPORTANT

**This package is currently WIP and NOT fully functional yet!**

Tuioxide is a Rust implementation of the [TUIO](https://tuio.org) specification by Martin Kaltenbrunner. It supports both [TUIO 1.1](https://tuio.org/?specification) and [TUIO 2.0](https://www.tuio.org/?tuio20) and is based on [rosc](https://crates.io/crates/rosc).

## Features
- TUIO 1.1 and TUIO 2.0
- Client and Server
- UDP and Websocket (as a feature)

# Getting Started

## Examples

All examples listen on localhost with port 3333

Receive TUIO 1.1 over udp
```sh
cargo run --example receive_tuio11_udp
```

Receive TUIO 1.1 over websocket
```sh
cargo run --example receive_tuio11_websocket --feature websocket
```

Receive TUIO 2.0 over udp
```sh
cargo run --example receive_tuio20_udp
```
