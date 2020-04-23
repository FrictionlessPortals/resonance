//! The collection of API endpoints for ``harmonic``.

use crate::{harmonic_response, types::BufferTx};
use futures::SinkExt;
use log::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use warp::ws::Message;

/// Generic Server Information
pub mod info {
    use super::*;

    // Main API Handshake Response
    harmonic_response!("API Handshake Response", HarmonicHandshake, version > &'static str, session > Uuid);

    // Main API Error Response
    harmonic_response!("API Error Response", HarmonicInvalid, reason > &'static str);
}
