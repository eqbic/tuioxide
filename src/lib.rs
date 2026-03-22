//! # tuioxide
//!
//! A Rust library for receiving and processing [TUIO](https://www.tuio.org/) messages
//! over UDP and WebSocket transports.
//!
//! TUIO is an open framework that defines a common protocol and API for tangible
//! multitouch surfaces. This crate supports both the **TUIO 1.1** and **TUIO 2.0**
//! protocol versions.
//!
//! ## Features
//!
//! - **TUIO 1.1**: cursors (`/tuio/2Dcur`), objects (`/tuio/2Dobj`), and blobs (`/tuio/2Dblb`).
//! - **TUIO 2.0**: pointers (`/tuio2/ptr`), tokens (`/tuio2/tok`), bounds (`/tuio2/bnd`),
//!   and symbols (`/tuio2/sym`).
//! - UDP transport (always available).
//! - WebSocket transport (enabled via the `websocket` feature flag).
//!
//! ## Crate Structure
//!
//! - [`core`] — Core data types, profiles, OSC decoding/encoding, and math primitives.
//! - [`tuio11`] — TUIO 1.1 client and event processing.
//! - [`tuio20`] — TUIO 2.0 client and event processing.
//!
//! ## Quick Start
//!
//! ```no_run
//! use std::net::Ipv4Addr;
//! use tuioxide::tuio11::Client;
//! use tuioxide::core::osc_receiver::{OscReceiver, UdpOscReceiver};
//!
//! let receiver = UdpOscReceiver::new(Ipv4Addr::LOCALHOST, 3333);
//! let mut client = Client::new(receiver);
//!
//! loop {
//!     if let Ok(events) = client.update() {
//!         for event in events.cursor_events {
//!             println!("{event:?}");
//!         }
//!     }
//! }
//! ```

pub mod core;
pub mod tuio11;
pub mod tuio20;
