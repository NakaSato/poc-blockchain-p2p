//! Network Infrastructure
//!
//! Provides abstraction over different network backends.

use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// Generic network provider trait
#[async_trait]
pub trait NetworkProvider: Send + Sync {
    async fn broadcast(&self, message: NetworkMessage) -> Result<(), DomainError>;
    async fn send_to_peer(&self, peer_id: String, message: NetworkMessage) -> Result<(), DomainError>;
    async fn get_connected_peers(&self) -> Result<Vec<PeerInfo>, DomainError>;
    async fn connect_to_peer(&self, address: String) -> Result<String, DomainError>; // Returns peer ID
    async fn disconnect_from_peer(&self, peer_id: String) -> Result<(), DomainError>;
}

/// Network message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub message_type: String,
    pub payload: Vec<u8>,
    pub sender_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl NetworkMessage {
    pub fn new(message_type: String, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload,
            sender_id: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn with_sender(mut self, sender_id: String) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub address: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// P2P Network adapter (placeholder for actual P2P implementation)
pub struct P2PNetworkAdapter {
    node_id: String,
    connected_peers: Arc<tokio::sync::RwLock<std::collections::HashMap<String, PeerInfo>>>,
}

impl P2PNetworkAdapter {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            connected_peers: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

#[async_trait]
impl NetworkProvider for P2PNetworkAdapter {
    async fn broadcast(&self, message: NetworkMessage) -> Result<(), DomainError> {
        let peers = self.connected_peers.read().await;
        
        // In a real implementation, this would broadcast to all connected peers
        for (peer_id, _peer_info) in peers.iter() {
            // Simulate sending message to each peer
            println!("Broadcasting message '{}' to peer: {}", message.message_type, peer_id);
        }
        
        Ok(())
    }

    async fn send_to_peer(&self, peer_id: String, message: NetworkMessage) -> Result<(), DomainError> {
        let peers = self.connected_peers.read().await;
        
        if peers.contains_key(&peer_id) {
            // In a real implementation, this would send to the specific peer
            println!("Sending message '{}' to peer: {}", message.message_type, peer_id);
            Ok(())
        } else {
            Err(DomainError::invalid_operation(format!("Peer not connected: {}", peer_id)))
        }
    }

    async fn get_connected_peers(&self) -> Result<Vec<PeerInfo>, DomainError> {
        let peers = self.connected_peers.read().await;
        Ok(peers.values().cloned().collect())
    }

    async fn connect_to_peer(&self, address: String) -> Result<String, DomainError> {
        // Generate a mock peer ID
        let peer_id = format!("peer_{}", uuid::Uuid::new_v4());
        
        let peer_info = PeerInfo {
            peer_id: peer_id.clone(),
            address: address.clone(),
            connected_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
        };
        
        let mut peers = self.connected_peers.write().await;
        peers.insert(peer_id.clone(), peer_info);
        
        println!("Connected to peer: {} at address: {}", peer_id, address);
        Ok(peer_id)
    }

    async fn disconnect_from_peer(&self, peer_id: String) -> Result<(), DomainError> {
        let mut peers = self.connected_peers.write().await;
        
        if peers.remove(&peer_id).is_some() {
            println!("Disconnected from peer: {}", peer_id);
            Ok(())
        } else {
            Err(DomainError::invalid_operation(format!("Peer not found: {}", peer_id)))
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub node_id: String,
    pub listen_address: String,
    pub bootstrap_peers: Vec<String>,
    pub max_connections: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            node_id: format!("node_{}", uuid::Uuid::new_v4()),
            listen_address: "0.0.0.0:8000".to_string(),
            bootstrap_peers: Vec::new(),
            max_connections: 50,
        }
    }
}

/// Network factory
pub struct NetworkFactory;

impl NetworkFactory {
    pub fn create_network(config: &NetworkConfig) -> Result<Arc<dyn NetworkProvider>, DomainError> {
        let network = P2PNetworkAdapter::new(config.node_id.clone());
        Ok(Arc::new(network))
    }
}
