//! Derivation Pipeline Configuration
//!
//! ## Build Explicit Config
//!
//! ```rust
//! use axos::config::Config;
//!
//! let config = Config {
//!    l1_rpc_url: "http://localhost:9933".to_string(),
//!    l2_rpc_url: "http://localhost:9934".to_string(),
//!
//! };
//! assert_eq!(config.chain.network, "mainnet");
//! ```
//!
//! ## Build Config from Environment Variables
//!
#![cfg_attr(
    feature = "alloc",
    doc = "
```rust
use axos::config::Config;
use std::env;
env::set_var('AXOS_L1_RPC_URL', 'http://localhost:9933');
env::set_var('AXOS_L2_RPC_URL', 'http://localhost:9934');
let config = Config::from_env();
assert_eq!(config.chain.network, 'mainnet');
```
"
)]
//!
//! ## Serializing Config
//!
// #![cfg_attr(feature = "serde", doc = "
// ```rust
// use axos::config::Config;
// use serde_json;
// let config = Config {
//     l1_rpc_url: 'http://localhost:9933'.to_string(),
//     l2_rpc_url: 'http://localhost:9934'.to_string(),
// };
// let json = serde_json::to_string(&config).unwrap();
// assert_eq!(json, r#"{\"l1_rpc_url\":\"http://localhost:9933\",\"l2_rpc_url\":\"http://localhost:9934\",\"chain\":{\"network\":\"mainnet\",\"l1_start_epoch\":0,\"l2_genesis\":{\"hash\":\"0x0000000
// ```
// ")]

#[cfg(feature = "alloc")]
use alloc::string::String;
use axos_primitives::ChainConfig;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Derivation Settings
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Config {
    /// The base chain RPC URL
    #[cfg(feature = "alloc")]
    pub l1_rpc_url: String,
    #[cfg(not(feature = "alloc"))]
    pub l1_rpc_url: &'static str,
    /// The L2 chain RPC URL
    #[cfg(feature = "alloc")]
    pub l2_rpc_url: String,
    #[cfg(not(feature = "alloc"))]
    pub l2_rpc_url: &'static str,
    /// The L2 engine API URL
    #[cfg(feature = "alloc")]
    pub l2_engine_url: String,
    #[cfg(not(feature = "alloc"))]
    pub l2_engine_url: &'static str,
    /// The base chain config
    pub chain: ChainConfig,
    /// Engine API JWT Secret
    /// This is used to authenticate with the engine API
    #[cfg(feature = "alloc")]
    pub jwt_secret: String,
    #[cfg(not(feature = "alloc"))]
    pub jwt_secret: &'static str,
    /// A trusted L2 RPC URL to use for fast/checkpoint syncing
    #[cfg(feature = "alloc")]
    pub checkpoint_sync_url: Option<String>,
    #[cfg(not(feature = "alloc"))]
    pub checkpoint_sync_url: Option<&'static str>,
    /// The port of RPC server
    pub rpc_port: u16,
    /// The devnet mode.
    pub devnet: bool,
}
