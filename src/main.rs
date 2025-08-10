//! GridTokenX Blockchain - Main Entry Point
//!
//! A revolutionary blockchain-based platform that enables peer-to-peer energy trading
//! in Thailand's electricity market.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

// Use the library exports instead of local modules
use gridtokenx_blockchain::{Blockchain, Block, Transaction, NodeConfig, StorageManager, ValidatorInfo, crypto};

#[derive(Parser)]
#[command(name = "gridtokenx-node")]
#[command(about = "GridTokenX Blockchain Node - P2P Energy Trading Platform")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the blockchain node
    Start {
        /// Configuration file path
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        /// Enable mining
        #[arg(short, long)]
        mining: bool,
        /// Node type (validator, trader, observer)
        #[arg(short, long, default_value = "validator")]
        node_type: String,
    },
    /// Initialize a new blockchain
    Init {
        /// Genesis block configuration
        #[arg(short, long)]
        genesis_config: Option<String>,
    },
    /// Show node status
    Status,
    /// Generate new wallet
    GenerateWallet,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start {
            config,
            mining,
            node_type,
        }) => {
            start_node(config, mining, node_type).await?;
        }
        Some(Commands::Init { genesis_config }) => {
            init_blockchain(genesis_config).await?;
        }
        Some(Commands::Status) => {
            show_status().await?;
        }
        Some(Commands::GenerateWallet) => {
            generate_wallet().await?;
        }
        None => {
            // Default: start node with default config
            start_node("config.toml".to_string(), false, "validator".to_string()).await?;
        }
    }

    Ok(())
}

async fn start_node(config_path: String, enable_mining: bool, node_type: String) -> Result<()> {
    info!("Starting GridTokenX Blockchain Node...");

    // Load configuration
    let config = NodeConfig::load(&config_path).unwrap_or_else(|_| {
        info!("Using default configuration");
        NodeConfig::default()
    });
    info!("Configuration loaded");

    // Initialize storage
    let storage = Arc::new(StorageManager::new(&config.storage.path).await?);
    info!("Storage initialized at: {}", config.storage.path);

    // Initialize blockchain
    let blockchain = Arc::new(RwLock::new(Blockchain::new(storage.clone()).await?));
    info!("Blockchain initialized");

    // Check if genesis block exists
    let height = {
        let bc = blockchain.read().await;
        bc.get_height().await.unwrap_or(0)
    };

    if height == 0 {
        info!("No genesis block found, creating one...");
        let genesis_block = create_genesis_block(None).await?;
        let mut bc = blockchain.write().await;
        bc.add_genesis_block(genesis_block).await?;
        info!("Genesis block created");
    }

    info!("GridTokenX Node started successfully!");
    info!("Node Type: {}", node_type);
    info!("Mining enabled: {}", enable_mining);
    info!("Current blockchain height: {}", {
        let bc = blockchain.read().await;
        bc.get_height().await.unwrap_or(0)
    });

    // Main event loop
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        // Mine a block if mining is enabled
        if enable_mining {
            let blockchain_clone = blockchain.clone();
            
            tokio::spawn(async move {
                if let Err(e) = mine_block(blockchain_clone).await {
                    error!("Mining error: {}", e);
                }
            });
        }

        // Simulate some transactions for demonstration
        if enable_mining {
            tokio::spawn(async move {
                if let Err(e) = simulate_transactions().await {
                    error!("Transaction simulation error: {}", e);
                }
            });
        }
    }
}

/// Simple mining function
async fn mine_block(blockchain: Arc<RwLock<Blockchain>>) -> Result<()> {
    let pending_transactions = {
        let bc = blockchain.read().await;
        bc.get_pending_transactions(100).await
    };

    if !pending_transactions.is_empty() {
        info!("Mining block with {} transactions", pending_transactions.len());
        
        let latest_block = {
            let bc = blockchain.read().await;
            bc.get_latest_block().await?
        };

        let validator_info = ValidatorInfo {
            address: "miner".to_string(),
            stake: 0,
            reputation: 50.0,
            authority_type: Some("MINER".to_string()),
        };

        let new_block = Block::new(
            latest_block.header.hash,
            pending_transactions.clone(),
            latest_block.header.height + 1,
            validator_info,
        )?;

        let bc = blockchain.read().await;
        bc.add_block(new_block.clone()).await?;

        let tx_ids: Vec<String> = pending_transactions
            .iter()
            .map(|tx| tx.id.clone())
            .collect();
        bc.remove_pending_transactions(&tx_ids).await;

        info!("Mined block at height: {}", new_block.header.height);
    }

    Ok(())
}

/// Simple transaction simulation for demonstration
async fn simulate_transactions() -> Result<()> {
    info!("Simulating transactions...");
    // This is just a placeholder for transaction simulation
    // In a real implementation, this would create and submit transactions
    Ok(())
}

async fn init_blockchain(genesis_config: Option<String>) -> Result<()> {
    info!("Initializing new GridTokenX blockchain...");

    let config = NodeConfig::default();
    let storage = Arc::new(StorageManager::new(&config.storage.path).await?);

    // Create genesis block
    let genesis_block = create_genesis_block(genesis_config).await?;

    // Initialize blockchain with genesis block
    let mut blockchain = Blockchain::new(storage).await?;
    blockchain.add_genesis_block(genesis_block).await?;

    info!("Blockchain initialized successfully!");
    info!("Genesis block created with Thai energy market parameters");

    Ok(())
}

async fn create_genesis_block(_genesis_config: Option<String>) -> Result<Block> {
    info!("Creating genesis block...");

    // Default genesis configuration for Thai energy market
    let genesis_transactions = vec![
        // Initial token distribution
        Transaction::new_genesis_mint(
            "system".to_string(),
            1_000_000_000, // 1 billion tokens initial supply
            "Initial token supply for Thai energy market".to_string(),
        )?,
        // Register energy authorities
        Transaction::new_authority_registration(
            "EGAT".to_string(), // Electricity Generating Authority of Thailand
            "Primary electricity generator".to_string(),
        )?,
        Transaction::new_authority_registration(
            "MEA".to_string(), // Metropolitan Electricity Authority
            "Bangkok and surrounding areas distribution".to_string(),
        )?,
        Transaction::new_authority_registration(
            "PEA".to_string(), // Provincial Electricity Authority
            "Provincial electricity distribution".to_string(),
        )?,
    ];

    let genesis_block = Block::new_genesis(
        genesis_transactions,
        "GridTokenX Genesis Block - Thai Energy Market".to_string(),
    )?;

    info!(
        "Genesis block created with {} transactions",
        genesis_block.transactions.len()
    );

    Ok(genesis_block)
}

async fn show_status() -> Result<()> {
    println!("GridTokenX Blockchain Node Status");
    println!("================================");

    // Try to load local blockchain state
    let config = NodeConfig::default();

    match StorageManager::new(&config.storage.path).await {
        Ok(storage) => {
            let storage = Arc::new(storage);
            match Blockchain::new(storage).await {
                Ok(blockchain) => {
                    let height = blockchain.get_height().await.unwrap_or(0);
                    let total_transactions = blockchain.get_total_transactions().await.unwrap_or(0);

                    println!("Node Status: Initialized");
                    println!("Blockchain Height: {}", height);
                    println!("Total Transactions: {}", total_transactions);

                    if let Ok(latest_block) = blockchain.get_latest_block().await {
                        println!("Latest Block Hash: {}", latest_block.header.hash);
                        println!("Latest Block Time: {}", latest_block.header.timestamp);
                    }
                }
                Err(e) => {
                    println!("Node Status: Error - Failed to load blockchain: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Node Status: Error - Failed to initialize storage: {}", e);
        }
    }

    Ok(())
}

async fn generate_wallet() -> Result<()> {
    info!("Generating new GridTokenX wallet...");

    let wallet = crypto::generate_keypair()?;

    println!("New GridTokenX Wallet Generated");
    println!("==============================");
    println!("Public Key: {}", hex::encode(&wallet.public_key));
    println!("Address: {}", wallet.address);
    println!("");
    println!("Private Key: {}", hex::encode(&wallet.private_key));
    println!("");
    println!("⚠️  IMPORTANT: Keep your private key secure and never share it!");
    println!("⚠️  This private key controls access to your energy tokens.");

    Ok(())
}

