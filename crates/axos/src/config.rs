//! Derivation Pipeline Configuration
//!
//! ## Build Explicit Config
//!
//! ```rust
//! use axos::config::Config;
//!
//! let l1_rpc_url = "http://localhost:9933".to_string();
//! let l2_rpc_url = "http://localhost:9934".to_string();
//! let config = Config {
//!    l1_rpc_url: l1_rpc_url.clone(),
//!    l2_rpc_url: l2_rpc_url.clone(),
//!    ..Default::default()
//! };
//! assert_eq!(config.l1_rpc_url, l1_rpc_url);
//! assert_eq!(config.l2_rpc_url, l2_rpc_url);
//! assert_eq!(config.chain.network, "optimism");
//! ```
//!
//! ## Build Config from Environment Variables
//!
#![cfg_attr(
    feature = "std",
    doc = "
```rust
use axos::config::Config;
use std::env;
env::set_var(\"AXOS_L1_RPC_URL\", \"http://localhost:9933\");
env::set_var(\"AXOS_L2_RPC_URL\", \"http://localhost:9934\");
let config = Config::from_env();
assert_eq!(config.l1_rpc_url, \"http://localhost:9933\");
assert_eq!(config.l2_rpc_url, \"http://localhost:9934\");
assert_eq!(config.chain.network, \"optimism\");
```
"
)]
//!
//! ## Serializing Config
//!
#![cfg_attr(
    feature = "serde",
    doc = "
```rust
use axos::config::Config;
use serde_json;
let config = Config {
    l1_rpc_url: \"http://localhost:9933\".to_string(),
    l2_rpc_url: \"http://localhost:9934\".to_string(),
    ..Default::default()
};
let json = serde_json::to_string(&config).unwrap();
let deserialized: Config = serde_json::from_str(&json).unwrap();
assert_eq!(deserialized, config);
```
"
)]

use axos_primitives::ChainConfig;
use axos_primitives::GenericString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Derivation Settings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Config {
    /// The base chain RPC URL
    pub l1_rpc_url: GenericString,
    /// The L2 chain RPC URL
    pub l2_rpc_url: GenericString,
    /// The L2 engine API URL
    pub l2_engine_url: GenericString,
    /// The base chain config
    pub chain: ChainConfig,
    /// Engine API JWT Secret
    /// This is used to authenticate with the engine API
    pub jwt_secret: GenericString,
    /// A trusted L2 RPC URL to use for fast/checkpoint syncing
    pub checkpoint_sync_url: Option<GenericString>,
    /// The port of RPC server
    pub rpc_port: u16,
    /// The devnet mode.
    pub devnet: bool,
}

#[cfg(feature = "std")]
impl Config {
    /// Build a config from environment variables.
    pub fn from_env() -> Self {
        let l1_rpc_url = std::env::var("AXOS_L1_RPC_URL")
            .expect("AXOS_L1_RPC_URL environment variable not set.");
        let l2_rpc_url = std::env::var("AXOS_L2_RPC_URL")
            .expect("AXOS_L2_RPC_URL environment variable not set.");
        let l2_engine_url = std::env::var("AXOS_L2_ENGINE_URL")
            .expect("AXOS_L2_ENGINE_URL environment variable not set.");
        let jwt_secret = std::env::var("AXOS_JWT_SECRET")
            .expect("AXOS_JWT_SECRET environment variable not set.");
        let checkpoint_sync_url = std::env::var("AXOS_CHECKPOINT_SYNC_URL").ok();
        let rpc_port = std::env::var("AXOS_RPC_PORT")
            .expect("AXOS_RPC_PORT environment variable not set.")
            .parse::<u16>()
            .expect("AXOS_RPC_PORT environment variable is not a valid port number.");
        let devnet = std::env::var("AXOS_DEVNET")
            .expect("AXOS_DEVNET environment variable not set.")
            .parse::<bool>()
            .expect("AXOS_DEVNET environment variable is not a valid boolean.");

        Self {
            l1_rpc_url,
            l2_rpc_url,
            l2_engine_url,
            jwt_secret,
            checkpoint_sync_url,
            rpc_port,
            devnet,
            ..Default::default()
        }
    }
}
