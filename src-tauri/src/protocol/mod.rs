//! Protocol abstraction layer for Spoky
//!
//! Provides the trait interface and event system for chat protocol integrations.

pub mod adapter;
pub mod events;
pub mod registry;

use thiserror::Error;

/// Standardized errors for protocol operations
#[derive(Error, Debug, Clone)]
pub enum ProtocolError {
    #[error("Not authenticated")]
    NotAuthenticated,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Rate limited")]
    RateLimited,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Protocol not supported")]
    NotSupported,
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Message send failed: {0}")]
    SendFailed(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl serde::Serialize for ProtocolError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
