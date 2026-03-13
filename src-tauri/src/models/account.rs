use crate::models::{ConnectionStatus, Protocol};
use serde::{Deserialize, Serialize};

/// Account data model representing a connected protocol account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub protocol: Protocol,
    pub credentials: String, // encrypted credentials JSON
    pub display_name: Option<String>,
    pub connection_status: ConnectionStatus,
    pub created_at: i64, // Unix timestamp in milliseconds
    pub updated_at: i64, // Unix timestamp in milliseconds
}

/// New account data for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAccount {
    pub protocol: Protocol,
    pub credentials: String,
    pub display_name: Option<String>,
}

/// Account update data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountUpdate {
    pub display_name: Option<Option<String>>,
    pub connection_status: Option<ConnectionStatus>,
    pub credentials: Option<String>,
}

impl Account {
    /// Create a new account with generated ID and timestamps
    pub fn create(new: NewAccount) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            protocol: new.protocol,
            credentials: new.credentials,
            display_name: new.display_name,
            connection_status: ConnectionStatus::Disconnected,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update account fields
    pub fn update(&mut self, update: AccountUpdate) {
        let now = chrono::Utc::now().timestamp_millis();

        if let Some(display_name) = update.display_name {
            self.display_name = display_name;
        }

        if let Some(status) = update.connection_status {
            self.connection_status = status;
        }

        if let Some(credentials) = update.credentials {
            self.credentials = credentials;
        }

        self.updated_at = now;
    }

    /// Check if account is currently connected
    pub fn is_connected(&self) -> bool {
        self.connection_status == ConnectionStatus::Connected
    }
}
