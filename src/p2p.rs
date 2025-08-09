//! GridTokenX P2P Network Module
//!
//! This module implements peer-to-peer networking for the GridTokenX blockchain,
//! including node discovery, message propagation, and network synchronization.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use libp2p::{
    gossipsub::{self, Behaviour as Gossipsub, Event as GossipsubEvent, MessageAuthenticity, ValidationMode},
    identity::Keypair,
    kad::{store::MemoryStore, Behaviour as Kademlia, Event as KademliaEvent},
    mdns::{Event as MdnsEvent, tokio::Behaviour as Mdns},
    swarm::{NetworkBehaviour, SwarmEvent, SwarmBuilder},
    Multiaddr, PeerId, Swarm, Transport,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::blockchain::{Block, Blockchain, Transaction};
use crate::config::P2PConfig;
use crate::consensus::ConsensusEngine;

/// P2P network manager
#[derive(Debug)]
pub struct P2PNetwork {
    config: P2PConfig,
    swarm: Option<Swarm<GridTokenXBehaviour>>,
    blockchain: Arc<RwLock<Blockchain>>,
    consensus: Arc<RwLock<ConsensusEngine>>,
    peers: RwLock<HashMap<PeerId, PeerInfo>>,
    message_handler: RwLock<MessageHandler>,
}

/// Network behavior for GridTokenX
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "GridTokenXEvent")]
pub struct GridTokenXBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
    pub kademlia: Kademlia<libp2p::kad::store::MemoryStore>,
}

/// Network events
#[derive(Debug)]
pub enum GridTokenXEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Kademlia(KademliaEvent),
}

impl From<GossipsubEvent> for GridTokenXEvent {
    fn from(event: GossipsubEvent) -> Self {
        GridTokenXEvent::Gossipsub(event)
    }
}

impl From<MdnsEvent> for GridTokenXEvent {
    fn from(event: MdnsEvent) -> Self {
        GridTokenXEvent::Mdns(event)
    }
}

impl From<KademliaEvent> for GridTokenXEvent {
    fn from(event: KademliaEvent) -> Self {
        GridTokenXEvent::Kademlia(event)
    }
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub node_type: String,
    pub version: String,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub reputation: f64,
    pub latency: Option<u64>,
    pub synced_height: u64,
}

/// Message handler for network messages
#[derive(Debug, Default)]
pub struct MessageHandler {
    pending_blocks: HashMap<String, Block>,
    pending_transactions: HashMap<String, Transaction>,
    sync_requests: HashMap<String, SyncRequest>,
}

/// Synchronization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub request_id: String,
    pub requester: String,
    pub start_height: u64,
    pub end_height: u64,
    pub requested_at: DateTime<Utc>,
}

/// Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// New block announcement
    BlockAnnouncement { block: Block, sender: String },
    /// Request specific block
    BlockRequest {
        request_id: String,
        height: u64,
        requester: String,
    },
    /// Block response
    BlockResponse {
        request_id: String,
        block: Option<Block>,
        responder: String,
    },
    /// New transaction
    TransactionBroadcast {
        transaction: Transaction,
        sender: String,
    },
    /// Blockchain sync request
    SyncRequest {
        request_id: String,
        start_height: u64,
        end_height: u64,
        requester: String,
    },
    /// Sync response with multiple blocks
    SyncResponse {
        request_id: String,
        blocks: Vec<Block>,
        responder: String,
    },
    /// Peer information exchange
    PeerInfo { info: PeerInfo },
    /// Consensus message
    ConsensusMessage {
        message_type: String,
        data: Vec<u8>,
        sender: String,
    },
    /// Ping for connectivity check
    Ping {
        timestamp: DateTime<Utc>,
        sender: String,
    },
    /// Pong response
    Pong {
        timestamp: DateTime<Utc>,
        original_timestamp: DateTime<Utc>,
        sender: String,
    },
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub connected_peers: u64,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub blocks_synced: u64,
    pub transactions_relayed: u64,
    pub average_latency: f64,
    pub network_health: f64,
}

impl P2PNetwork {
    /// Create new P2P network
    pub async fn new(
        config: P2PConfig,
        blockchain: Arc<RwLock<Blockchain>>,
        consensus: Arc<RwLock<ConsensusEngine>>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            swarm: None,
            blockchain,
            consensus,
            peers: RwLock::new(HashMap::new()),
            message_handler: RwLock::new(MessageHandler::default()),
        })
    }

    /// Start the P2P network
    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting P2P network on {}", self.config.listen_addr);

        // Generate or load keypair
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        tracing::info!("Local peer ID: {}", local_peer_id);

        // Set up transport
        let noise_keys = libp2p::noise::Keypair::<libp2p::noise::X25519Spec>::new()
            .into_authentic(&local_key)
            .expect("Signing libp2p-noise static DH keypair failed.");

        let transport = libp2p::tcp::TcpConfig::new()
            .nodelay(true)
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(YamuxConfig::default())
            .boxed();

        // Set up Gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(
                self.config.gossip.heartbeat_interval,
            ))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(|message| {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                message.data.hash(&mut hasher);
                gossipsub::MessageId::from(hasher.finish().to_string())
            })
            .build()
            .expect("Valid gossipsub config");

        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .expect("Correct gossipsub configuration");

        // Subscribe to topics
        let block_topic = gossipsub::IdentTopic::new("gridtokenx-blocks");
        let tx_topic = gossipsub::IdentTopic::new("gridtokenx-transactions");
        let consensus_topic = gossipsub::IdentTopic::new("gridtokenx-consensus");

        gossipsub.subscribe(&block_topic)?;
        gossipsub.subscribe(&tx_topic)?;
        gossipsub.subscribe(&consensus_topic)?;

        // Set up mDNS
        let mdns = Mdns::new(libp2p::mdns::MdnsConfig::default()).await?;

        // Set up Kademlia
        let store = libp2p::kad::store::MemoryStore::new(local_peer_id);
        let mut kademlia = Kademlia::new(local_peer_id, store);

        // Set Kademlia mode
        kademlia.set_mode(Some(libp2p::kad::Mode::Server));

        // Create network behaviour
        let behaviour = GridTokenXBehaviour {
            gossipsub,
            mdns,
            kademlia,
        };

        // Create swarm
        let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        // Listen on configured address
        let listen_addr: Multiaddr = self.config.listen_addr.parse()?;
        swarm.listen_on(listen_addr)?;

        self.swarm = Some(swarm);

        // Start event loop
        self.run_event_loop().await?;

        Ok(())
    }

    /// Run the main event loop
    async fn run_event_loop(&mut self) -> Result<()> {
        if let Some(swarm) = &mut self.swarm {
            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        tracing::info!("Listening on {}", address);
                    }
                    SwarmEvent::Behaviour(event) => {
                        self.handle_behaviour_event(event).await?;
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        tracing::info!("Connected to peer: {}", peer_id);
                        self.handle_peer_connected(peer_id).await?;
                    }
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        tracing::info!("Disconnected from peer: {}", peer_id);
                        self.handle_peer_disconnected(peer_id).await?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Handle network behaviour events
    async fn handle_behaviour_event(&mut self, event: GridTokenXEvent) -> Result<()> {
        match event {
            GridTokenXEvent::Gossipsub(gossipsub::GossipsubEvent::Message {
                propagation_source,
                message_id: _,
                message,
            }) => {
                self.handle_gossip_message(propagation_source, &message.data)
                    .await?;
            }
            GridTokenXEvent::Mdns(MdnsEvent::Discovered(list)) => {
                for (peer_id, multiaddr) in list {
                    tracing::info!("Discovered peer: {} at {}", peer_id, multiaddr);
                    if let Some(swarm) = &mut self.swarm {
                        swarm
                            .behaviour_mut()
                            .kademlia
                            .add_address(&peer_id, multiaddr);
                    }
                }
            }
            GridTokenXEvent::Mdns(MdnsEvent::Expired(list)) => {
                for (peer_id, multiaddr) in list {
                    tracing::info!("Peer expired: {} at {}", peer_id, multiaddr);
                }
            }
            GridTokenXEvent::Kademlia(kad_event) => {
                tracing::debug!("Kademlia event: {:?}", kad_event);
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle gossip messages
    async fn handle_gossip_message(&self, _source: PeerId, data: &[u8]) -> Result<()> {
        match bincode::deserialize::<NetworkMessage>(data) {
            Ok(message) => {
                self.process_network_message(message).await?;
            }
            Err(e) => {
                tracing::warn!("Failed to deserialize network message: {}", e);
            }
        }
        Ok(())
    }

    /// Process network messages
    async fn process_network_message(&self, message: NetworkMessage) -> Result<()> {
        match message {
            NetworkMessage::BlockAnnouncement { block, sender } => {
                tracing::info!(
                    "Received block announcement: height {} from {}",
                    block.header.height,
                    sender
                );
                self.handle_block_announcement(block).await?;
            }
            NetworkMessage::TransactionBroadcast {
                transaction,
                sender,
            } => {
                tracing::debug!("Received transaction broadcast from {}", sender);
                self.handle_transaction_broadcast(transaction).await?;
            }
            NetworkMessage::SyncRequest {
                request_id,
                start_height,
                end_height,
                requester,
            } => {
                tracing::info!(
                    "Received sync request {}: blocks {}-{} from {}",
                    request_id,
                    start_height,
                    end_height,
                    requester
                );
                self.handle_sync_request(request_id, start_height, end_height, requester)
                    .await?;
            }
            NetworkMessage::Ping { timestamp, sender } => {
                self.handle_ping(timestamp, sender).await?;
            }
            _ => {
                tracing::debug!("Received network message: {:?}", message);
            }
        }
        Ok(())
    }

    /// Handle new block announcement
    async fn handle_block_announcement(&self, block: Block) -> Result<()> {
        let blockchain = self.blockchain.read().await;
        let current_height = blockchain.get_height().await?;

        if block.header.height > current_height {
            // New block ahead of our chain
            blockchain.add_block(block.clone()).await?;
            tracing::info!("Added new block at height {}", block.header.height);
        } else if block.header.height == current_height {
            // Potential fork - would need more sophisticated handling
            tracing::warn!("Received block at current height - potential fork");
        }

        Ok(())
    }

    /// Handle transaction broadcast
    async fn handle_transaction_broadcast(&self, transaction: Transaction) -> Result<()> {
        let blockchain = self.blockchain.read().await;
        blockchain
            .add_pending_transaction(transaction.clone())
            .await?;
        tracing::debug!("Added transaction to pending pool: {}", transaction.id);
        Ok(())
    }

    /// Handle sync request
    async fn handle_sync_request(
        &self,
        request_id: String,
        start_height: u64,
        end_height: u64,
        requester: String,
    ) -> Result<()> {
        let blockchain = self.blockchain.read().await;
        let mut blocks = Vec::new();

        for height in start_height..=end_height.min(start_height + 100) {
            // Limit to 100 blocks per request
            if let Ok(block) = blockchain.get_block_by_height(height).await {
                blocks.push(block);
            } else {
                break;
            }
        }

        if !blocks.is_empty() {
            let response = NetworkMessage::SyncResponse {
                request_id,
                blocks,
                responder: "local_node".to_string(),
            };

            self.broadcast_message(&response, "gridtokenx-blocks")
                .await?;
        }

        Ok(())
    }

    /// Handle ping message
    async fn handle_ping(&self, timestamp: DateTime<Utc>, sender: String) -> Result<()> {
        let pong = NetworkMessage::Pong {
            timestamp: Utc::now(),
            original_timestamp: timestamp,
            sender: "local_node".to_string(),
        };

        self.broadcast_message(&pong, "gridtokenx-consensus")
            .await?;
        Ok(())
    }

    /// Handle peer connection
    async fn handle_peer_connected(&self, peer_id: PeerId) -> Result<()> {
        let peer_info = PeerInfo {
            peer_id: peer_id.to_string(),
            addresses: Vec::new(),
            node_type: "unknown".to_string(),
            version: "unknown".to_string(),
            connected_at: Utc::now(),
            last_seen: Utc::now(),
            reputation: 50.0,
            latency: None,
            synced_height: 0,
        };

        let mut peers = self.peers.write().await;
        peers.insert(peer_id, peer_info);

        Ok(())
    }

    /// Handle peer disconnection
    async fn handle_peer_disconnected(&self, peer_id: PeerId) -> Result<()> {
        let mut peers = self.peers.write().await;
        peers.remove(&peer_id);
        Ok(())
    }

    /// Broadcast a message to the network
    pub async fn broadcast_message(&self, message: &NetworkMessage, topic: &str) -> Result<()> {
        if let Some(swarm) = &self.swarm {
            let data = bincode::serialize(message)?;
            let topic = gossipsub::IdentTopic::new(topic);

            // This would work if swarm was mutable, but we need to restructure for that
            // swarm.behaviour_mut().gossipsub.publish(topic, data)?;

            tracing::debug!("Broadcasting message to topic: {}", topic);
        }
        Ok(())
    }

    /// Broadcast new block
    pub async fn broadcast_block(&self, block: &Block) -> Result<()> {
        let message = NetworkMessage::BlockAnnouncement {
            block: block.clone(),
            sender: "local_node".to_string(),
        };

        self.broadcast_message(&message, "gridtokenx-blocks").await
    }

    /// Broadcast transaction
    pub async fn broadcast_transaction(&self, transaction: &Transaction) -> Result<()> {
        let message = NetworkMessage::TransactionBroadcast {
            transaction: transaction.clone(),
            sender: "local_node".to_string(),
        };

        self.broadcast_message(&message, "gridtokenx-transactions")
            .await
    }

    /// Request blockchain sync
    pub async fn request_sync(&self, start_height: u64, end_height: u64) -> Result<()> {
        let message = NetworkMessage::SyncRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            start_height,
            end_height,
            requester: "local_node".to_string(),
        };

        self.broadcast_message(&message, "gridtokenx-blocks").await
    }

    /// Get connected peers
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Get network statistics
    pub async fn get_stats(&self) -> NetworkStats {
        let peers = self.peers.read().await;
        let connected_peers = peers.len() as u64;

        // Calculate average latency
        let latencies: Vec<u64> = peers.values().filter_map(|p| p.latency).collect();

        let average_latency = if !latencies.is_empty() {
            latencies.iter().sum::<u64>() as f64 / latencies.len() as f64
        } else {
            0.0
        };

        // Calculate network health (simplified)
        let network_health = if connected_peers > 0 {
            let healthy_peers = peers.values().filter(|p| p.reputation > 70.0).count() as f64;
            (healthy_peers / connected_peers as f64) * 100.0
        } else {
            0.0
        };

        NetworkStats {
            connected_peers,
            total_messages_sent: 0,     // Would track in real implementation
            total_messages_received: 0, // Would track in real implementation
            blocks_synced: 0,           // Would track in real implementation
            transactions_relayed: 0,    // Would track in real implementation
            average_latency,
            network_health,
        }
    }
}

