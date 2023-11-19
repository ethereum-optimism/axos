#[cfg(feature = "alloc")]
use alloc::string::String;
use alloy_primitives::Address;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::blocks::BlockInfo;
use crate::epoch::Epoch;
use crate::system::SystemConfig;

mod base;
mod optimism;

/// A Chain Configuration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChainConfig {
    /// The network name
    #[cfg(feature = "alloc")]
    pub network: String,
    /// The network name (no-alloc)
    #[cfg(not(feature = "alloc"))]
    pub network: &'static str,
    /// The L1 chain id
    pub l1_chain_id: u64,
    /// The L2 chain id
    pub l2_chain_id: u64,
    /// The L1 block referenced by the L2 chain
    pub l1_start_epoch: Epoch,
    /// The L2 genesis block info
    pub l2_genesis: BlockInfo,
    /// The initial system config value
    pub system_config: SystemConfig,
    /// The batch inbox address
    pub batch_inbox: Address,
    /// The deposit contract address
    pub deposit_contract: Address,
    /// The L1 system config contract
    pub system_config_contract: Address,
    /// The maximum byte size of all pending channels
    pub max_channel_size: u64,
    /// The max timeout for a channel (as measured by the frame L1 block number)
    pub channel_timeout: u64,
    /// Number of L1 blocks in a sequence window
    pub seq_window_size: u64,
    /// Maximum timestamp drift
    pub max_seq_drift: u64,
    /// Timestamp of the regolith hardfork
    pub regolith_time: u64,
    /// Network blocktime
    #[serde(default = "default_blocktime")]
    pub blocktime: u64,
    /// L2 To L1 Message passer address
    pub l2_to_l1_message_passer: Address,
}

fn default_blocktime() -> u64 {
    2
}
