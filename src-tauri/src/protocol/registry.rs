use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::{Account, Protocol};
use super::adapter::{ProtocolAdapter, ProtocolAdapterFactory, ProtocolError};
use super::events::{ProtocolEvent, ConnectionStatus};

/// Manages active protocol adapter instances
pub struct ProtocolRegistry {
    /// Active adapters keyed by account_id
    adapters: Arc<RwLock<HashMap<String, Box<dyn ProtocolAdapter>>>>,
    /// Event channel sender - cloned to each adapter
    event_sender: tokio::sync::mpsc::Sender<ProtocolEvent>,
}

impl ProtocolRegistry {
    pub fn new(event_sender: tokio::sync::mpsc::Sender<ProtocolEvent>) -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
        }
    }

    /// Register and connect a protocol adapter for an account
    pub async fn connect_account(
        &self,
        account: &Account,
        factory: &dyn ProtocolAdapterFactory,
    ) -> Result<(), ProtocolError> {
        let mut adapter = factory.create();
        
        // Set up event forwarding
        let sender = self.event_sender.clone();
        adapter.on_event(Box::new(move |event| {
            // Send event through channel - don't block
            let _ = sender.try_send(event);
        }));

        // Connect to protocol
        adapter.connect(account).await?;

        // Store adapter
        let mut adapters = self.adapters.write().await;
        adapters.insert(account.id.clone(), adapter);

        Ok(())
    }

    /// Disconnect and remove a protocol adapter
    pub async fn disconnect_account(&self, account_id: &str) -> Result<(), ProtocolError> {
        let mut adapters = self.adapters.write().await;
        
        if let Some(mut adapter) = adapters.remove(account_id) {
            adapter.disconnect().await?;
        }

        Ok(())
    }

    /// Check if an account is connected
    pub async fn is_connected(&self, account_id: &str) -> bool {
        let adapters = self.adapters.read().await;
        adapters.contains_key(account_id)
    }

    /// Get connection status for an account
    pub async fn connection_status(
        &self,
        account_id: &str,
    ) -> Option<ConnectionStatus> {
        let adapters = self.adapters.read().await;
        adapters.get(account_id).map(|a| a.connection_status())
    }

    /// Get all connected account IDs
    pub async fn connected_accounts(&self) -> Vec<String> {
        let adapters = self.adapters.read().await;
        adapters.keys().cloned().collect()
    }
}

/// Stub adapter for testing the architecture
pub struct StubAdapter {
    protocol: Protocol,
    status: ConnectionStatus,
    event_callback: Option<Box<dyn Fn(ProtocolEvent) + Send + Sync>>,
}

impl StubAdapter {
    pub fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            status: ConnectionStatus::Disconnected,
            event_callback: None,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl ProtocolAdapter for StubAdapter {
    fn protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    async fn connect(
        &mut self,
        _account: &Account,
    ) -> Result<ConnectionStatus, ProtocolError> {
        self.status = ConnectionStatus::Connected;
        Ok(self.status.clone())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        self.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        self.status.clone()
    }

    async fn get_conversations(&self) -> Result<Vec<crate::models::Conversation>, ProtocolError> {
        Ok(vec![])
    }

    async fn get_messages(
        &self,
        _conversation_id: &str,
        _before: Option<chrono::DateTime<chrono::Utc>>,
        _limit: usize,
    ) -> Result<Vec<crate::models::Message>, ProtocolError> {
        Ok(vec![])
    }

    async fn send_message(
        &self,
        _conversation_id: &str,
        _content: &str,
    ) -> Result<crate::models::Message, ProtocolError> {
        Err(ProtocolError::NotAuthenticated)
    }

    fn on_event(&mut self,
        callback: Box<dyn Fn(ProtocolEvent) + Send + Sync>,
    ) {
        self.event_callback = Some(callback);
    }
}