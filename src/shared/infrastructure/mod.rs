//! Shared Infrastructure Components
//!
//! This module contains shared infrastructure components including
//! storage, networking, and external service adapters.

pub mod storage;
pub mod network;
pub mod logging;

// Re-export commonly used types
pub use storage::{StorageProvider, InMemoryStorage, FileSystemStorage};
pub use network::{NetworkProvider, P2PNetworkAdapter};
pub use logging::{Logger, StructuredLogger};
