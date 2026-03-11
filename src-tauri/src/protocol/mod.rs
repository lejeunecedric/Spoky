pub mod adapter;
pub mod events;
// pub mod registry; // TODO: Enable in Task 2

// Re-export public types
pub use adapter::{ProtocolAdapter, ProtocolAdapterFactory, ProtocolError};
pub use events::{ProtocolEvent, ConnectionEvent, MessageEvent, ConversationEvent, ProtocolErrorEvent, ConnectionStatus};
// pub use registry::ProtocolRegistry; // TODO: Enable in Task 2