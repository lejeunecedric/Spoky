//! Discord protocol implementation for Spoky
//!
//! Implements the ProtocolAdapter trait for Discord using the serenity crate.
//! Uses bot token authentication and Discord Gateway for real-time events.

pub mod adapter;
pub mod auth;
pub mod sync;

pub use adapter::DiscordAdapter;

use crate::protocol::ProtocolError;
use thiserror::Error;

/// Discord-specific errors
#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("Invalid bot token format")]
    InvalidToken,
    #[error("Discord API error: {0}")]
    ApiError(String),
    #[error("Gateway connection failed: {0}")]
    GatewayError(String),
    #[error("Rate limited: retry after {0}s")]
    RateLimited(u64),
    #[error("Channel not found: {0}")]
    ChannelNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl From<DiscordError> for ProtocolError {
    fn from(e: DiscordError) -> Self {
        match e {
            DiscordError::InvalidToken => ProtocolError::InvalidCredentials,
            DiscordError::ApiError(msg) => ProtocolError::ProtocolError(msg),
            DiscordError::GatewayError(msg) => ProtocolError::ConnectionFailed(msg),
            DiscordError::RateLimited(_) => ProtocolError::RateLimited,
            DiscordError::ChannelNotFound(msg) => ProtocolError::ProtocolError(msg),
            DiscordError::PermissionDenied(msg) => ProtocolError::ProtocolError(msg),
        }
    }
}

impl From<serenity::Error> for DiscordError {
    fn from(e: serenity::Error) -> Self {
        match e {
            serenity::Error::Http(http_err) => {
                if let serenity::http::error::ErrorType::UnsuccessfulRequest(response) = &http_err {
                    if response.status_code == 429 {
                        return DiscordError::RateLimited(
                            response.retry_after.unwrap_or(0) as u64
                        );
                    }
                    if response.status_code == 401 {
                        return DiscordError::InvalidToken;
                    }
                    if response.status_code == 403 {
                        return DiscordError::PermissionDenied(
                            response.error.message.clone()
                        );
                    }
                }
                DiscordError::ApiError(http_err.to_string())
            }
            serenity::Error::Gateway(msg) => DiscordError::GatewayError(msg),
            _ => DiscordError::ApiError(e.to_string()),
        }
    }
}
