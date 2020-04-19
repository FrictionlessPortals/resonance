//! # harmonic
//!
//! The API implementations and definitions for ``resonance``.
//!
//! ## Safety
//!
//! This library declares ``#![forbid(unsafe_code)]``.

#![forbid(unsafe_code)]

/// Harmonic API Version
pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Useful Custom Types
pub mod types {
    use futures::{channel::mpsc::UnboundedSender, stream::SplitSink};
    use warp::{
        ws::{Message, WebSocket},
        Error,
    };

    /// General Custom Types
    pub type WebSocketTx = SplitSink<WebSocket, Message>;
    pub type BufferTx = UnboundedSender<Result<Message, Error>>;
}

// Useful macros for generating endpoints.
mod macros;
pub use macros::*;

// The collection of API endpoints.
pub mod endpoints;
