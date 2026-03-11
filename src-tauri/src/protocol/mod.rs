pub mod adapter;
pub mod events;
pub mod registry;

// Re-export public types
pub use adapter::{ProtocolAdapter, ProtocolAdapterFactory, ProtocolError};
pub use events::{ProtocolEvent, ConnectionEvent, MessageEvent, ConversationEvent, ProtocolErrorEvent, ConnectionStatus};
pub use registry::{ProtocolRegistry, StubAdapter};