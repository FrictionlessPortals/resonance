//! The session handler for ``harmonic``.

use dashmap::DashMap;
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

/// Session Storage
///
/// Interally this uses ``DashMap`` for session storage.
#[derive(Debug, Clone)]
pub struct Session<S: Serialize + PartialEq + Debug> {
    /// Session Storage
    inner: DashMap<Uuid, S>,
}

impl<S: Serialize + Clone + PartialEq + Debug> Session<S> {
    /// Create a new session storage object.
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }

    /// Create a new session from given data.
    pub fn new_session(&self, data: S) -> Uuid {
        // Generate a new session UUID.
        let uuid = Uuid::new_v4();
        self.inner.insert(uuid, data);
        uuid
    }

    /// Check if the given session exists in storage.
    pub fn check_session(&self, key: Uuid) -> bool {
        self.inner.contains_key(&key)
    }

    /// Get the session data given session key.
    pub fn get_session_data(&self, key: Uuid) -> Option<S> {
        match self.inner.get(&key) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    /// Get the session key given session data.
    pub fn get_session_key(&self, data: S) -> Option<Uuid> {
        match self.inner.iter().find(|x| *x.value() == data) {
            Some(x) => Some(x.key().to_owned()),
            None => None,
        }
    }

    /// Remove the given session from storage.
    pub fn remove_session(&self, key: Uuid) -> Option<(Uuid, S)> {
        self.inner.remove(&key)
    }

    /// Print inner session object storage.
    pub fn print_sessions(&self) -> String {
        format!("Session Storage: {:?}", self.inner)
    }

    /// Expose the inner session storage object.
    pub fn into_inner(self) -> DashMap<Uuid, S> {
        self.inner
    }
}
