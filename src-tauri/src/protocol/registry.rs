//! Protocol registry for managing active connections
//!
//! Maintains a registry of connected protocol adapters with thread-safe access.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::AppHandle;
use crate::models::{Account, Conversation, Message};
use crate::protocol::adapter::{ProtocolAdapter, ProtocolAdapterFactory};
use crate::protocol::events::{ProtocolEvent, ConnectionStatus};
use crate::protocol::ProtocolError;

/// Manages active protocol adapter instances
pub struct ProtocolRegistry {
    /// Active adapters keyed by account_id
    adapters: Arc<RwLock<HashMap<String, Box<dyn ProtocolAdapter>>>>,
    /// Event channel sender - cloned to each adapter
    event_sender: tokio::sync::mpsc::Sender<ProtocolEvent>,
}

impl ProtocolRegistry {
    /// Create a new protocol registry
    pub fn new(
        event_sender: tokio::sync::mpsc::Sender<ProtocolEvent>,
    ) -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
        }
    }

    /// Register and connect a protocol adapter for an account
    pub async fn connect_account(
        &self,
        app_handle: AppHandle,
        account: &Account,
        factory: &dyn ProtocolAdapterFactory,
    ) -> Result<(), ProtocolError> {
        let mut adapter = factory.create();

        // Set up event forwarding
        let sender = self.event_sender.clone();
        let account_id = account.id.clone();
        adapter.on_event(Box::new(move |event| {
            // Send event through channel - don't block
            let _ = sender.try_send(event);
        }));

        // Set app handle for direct Tauri event emission
        adapter.set_app_handle(app_handle);

        // Connect to protocol
        log::info!(
            "Connecting account {} to {:?}",
            account_id,
            account.protocol
        );

        adapter.connect(account).await?;

        // Store adapter
        let mut adapters = self.adapters.write().await;
        adapters.insert(account_id.clone(), adapter);

        log::info!("Account {} connected successfully", account_id);
        Ok(())
    }

    /// Disconnect and remove a protocol adapter
    pub async fn disconnect_account(
        &self,
        account_id: &str,
    ) -> Result<(), ProtocolError> {
        let mut adapters = self.adapters.write().await;

        if let Some(mut adapter) = adapters.remove(account_id) {
            log::info!("Disconnecting account {}", account_id);
            adapter.disconnect().await?;
            log::info!("Account {} disconnected", account_id);
        } else {
            log::warn!(
                "Attempted to disconnect unknown account {}",
                account_id
            );
        }

        Ok(())
    }

    /// Get an adapter for an account
    pub async fn get_adapter(
        &self,
        account_id: &str,
    ) -> Option<impl std::ops::Deref<Target = dyn ProtocolAdapter> + '_> {
        let adapters = self.adapters.read().await;
        adapters.get(account_id).map(|a| a.as_ref())
    }

    /// Check if an account is connected
    pub async fn is_connected(
        &self,
        account_id: &str,
    ) -> bool {
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

    /// Get count of connected accounts
    pub async fn connected_count(&self) -> usize {
        let adapters = self.adapters.read().await;
        adapters.len()
    }

    /// Disconnect all accounts
    pub async fn disconnect_all(&self) -> Result<(), ProtocolError> {
        let account_ids: Vec<String> = {
            let adapters = self.adapters.read().await;
            adapters.keys().cloned().collect()
        };

        for account_id in account_ids {
            if let Err(e) = self.disconnect_account(&account_id).await {
                log::error!(
                    "Failed to disconnect account {}: {}",
                    account_id, e
                );
            }
        }

        Ok(())
    }
}

/// Stub adapter for testing the architecture
pub struct StubAdapter {
    protocol: crate::models::Protocol,
    status: ConnectionStatus,
    event_callback: Option<Box<dyn Fn(ProtocolEvent) + Send + Sync>>,
    account_id: Option<String>,
}

impl StubAdapter {
    /// Create a new stub adapter
    pub fn new(protocol: crate::models::Protocol) -> Self {
        Self {
            protocol,
            status: ConnectionStatus::Disconnected,
            event_callback: None,
            account_id: None,
        }
    }

    /// Trigger a test event
    pub fn trigger_test_event(&self,
    ) {
        if let Some(ref callback) = self.event_callback {
            let event = ProtocolEvent::ConnectionChanged(
                super::events::ConnectionEvent {
                    account_id: self
                        .account_id
                        .clone()
                        .unwrap_or_default(),
                    protocol: self.protocol.clone(),
                    status: self.status.clone(),
                    message: Some("Test event from stub".to_string()),
                },
            );
            callback(event);
        }
    }
}

#[async_trait::async_trait]
impl ProtocolAdapter for StubAdapter {
    fn protocol(&self,
    ) -> crate::models::Protocol {
        self.protocol.clone()
    }

    fn set_app_handle(&mut self, _app_handle: AppHandle) {
        // Stub adapter doesn't use app handle
    }

    async fn connect(
        &mut self,
        account: &Account,
    ) -> Result<ConnectionStatus, ProtocolError> {
        self.account_id = Some(account.id.clone());
        self.status = ConnectionStatus::Connected;

        // Emit connected event
        if let Some(ref callback) = self.event_callback {
            callback(ProtocolEvent::ConnectionChanged(
                super::events::ConnectionEvent {
                    account_id: account.id.clone(),
                    protocol: self.protocol.clone(),
                    status: ConnectionStatus::Connected,
                    message: None,
                },
            ));
        }

        Ok(self.status.clone())
    }

    async fn disconnect(
        &mut self,
    ) -> Result<(), ProtocolError> {
        self.status = ConnectionStatus::Disconnected;

        // Emit disconnected event
        if let Some(ref callback) = self.event_callback {
            callback(ProtocolEvent::ConnectionChanged(
                super::events::ConnectionEvent {
                    account_id: self
                        .account_id
                        .clone()
                        .unwrap_or_default(),
                    protocol: self.protocol.clone(),
                    status: ConnectionStatus::Disconnected,
                    message: None,
                },
            ));
        }

        self.account_id = None;
        Ok(())
    }

    fn connection_status(&self,
    ) -> ConnectionStatus {
        self.status.clone()
    }

    async fn get_conversations(
        &self,
    ) -> Result<Vec<Conversation>, ProtocolError> {
        Ok(vec![])
    }

    async fn get_messages(
        &self,
        _conversation_id: &str,
        _before: Option<i64>,
        _limit: usize,
    ) -> Result<Vec<Message>, ProtocolError> {
        Ok(vec![])
    }

    async fn send_message(
        &self,
        _conversation_id: &str,
        _content: &str,
    ) -> Result<Message, ProtocolError> {
        Err(ProtocolError::NotAuthenticated)
    }

    fn on_event(
        &mut self,
        callback: Box<dyn Fn(ProtocolEvent) + Send + Sync>,
    ) {
        self.event_callback = Some(callback);
    }

    fn account_id(&self) -> Option<&str> {
        self.account_id.as_deref()
    }
}

/// Factory for creating stub adapters
pub struct StubAdapterFactory {
    protocol: crate::models::Protocol,
}

impl StubAdapterFactory {
    /// Create a new stub factory
    pub fn new(protocol: crate::models::Protocol) -> Self {
        Self { protocol }
    }
}

impl ProtocolAdapterFactory for StubAdapterFactory {
    fn create(&self,
    ) -> Box<dyn ProtocolAdapter> {
        Box::new(StubAdapter::new(self.protocol.clone()))
    }

    fn protocol(&self,
    ) -> crate::models::Protocol {
        self.protocol.clone()
    }
}
