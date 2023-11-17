use alloc::string::String;
use axos_primitives::ChainConfig;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Derivation Settings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Config {
    /// The base chain RPC URL
    pub l1_rpc_url: String,
    /// The L2 chain RPC URL
    pub l2_rpc_url: String,
    /// The L2 engine API URL
    pub l2_engine_url: String,
    /// The base chain config
    pub chain: ChainConfig,
    /// Engine API JWT Secret
    /// This is used to authenticate with the engine API
    pub jwt_secret: String,
    /// A trusted L2 RPC URL to use for fast/checkpoint syncing
    pub checkpoint_sync_url: Option<String>,
    /// The port of RPC server
    pub rpc_port: u16,
    /// The devnet mode.
    pub devnet: bool,
}
