//! The collection of API endpoints for ``harmonic``.

use crate::harmonic_response;
use uuid::Uuid;

/// Generic Server Information
pub mod info {
    use super::*;

    harmonic_response!("API Handshake Endpoint", HarmonicHandshake, version > &'static str, session > Uuid);
}
