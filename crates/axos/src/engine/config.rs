//! Engine Config

use axos_primitives::GenericString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The Engine Config
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// The base chain block time.
    /// This is taken from the top-level axos derivation pipeline config
    /// in its [ChainConfig][axos_primitives::ChainConfig].
    pub blocktime: u64,
    /// The L2 engine API URL
    pub l2_engine_url: GenericString,
    /// Engine API JWT Secret
    /// This is used to authenticate with the engine API
    pub jwt_secret: GenericString,
}

impl From<axos_config::Config> for EngineConfig {
    fn from(config: axos_config::Config) -> Self {
        Self {
            blocktime: config.chain.blocktime,
            l2_engine_url: config.l2_engine_url,
            jwt_secret: config.jwt_secret,
        }
    }
}

impl From<&axos_config::Config> for EngineConfig {
    fn from(config: &axos_config::Config) -> Self {
        Self {
            blocktime: config.chain.blocktime,
            l2_engine_url: config.l2_engine_url.clone(),
            jwt_secret: config.jwt_secret.clone(),
        }
    }
}
