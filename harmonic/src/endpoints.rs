//! The collection of API endpoints for ``harmonic``.

use crate::harmonic_response;

/// Generic Server Information
pub mod info {
    use super::*;

    harmonic_response!("API Version Endpoint", HarmonicVersion, version > &'static str);
}
