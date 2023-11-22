//! Engine Driver
//!
//! This module was largely built based off [magi][magi].
//!
//! [magi]: https://github.com/a16z/magi

use crate::engine::api::EngineApi;

use anyhow::Result;

use axos_providers::mock::MockProvider;
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

use axos_primitives::{BlockInfo, Epoch};

use super::config::EngineConfig;

// todo: replace the hardcoded MockProvider below
// with the generic provider trait.

/// The engine driver
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EngineDriver {
    /// The L2 execution engine
    engine: EngineApi,
    /// Provider for the local L2 execution RPC
    provider: MockProvider,
    /// Blocktime of the L2 chain
    blocktime: u64,
    /// Most recent block found on the p2p network
    pub unsafe_head: BlockInfo,
    /// Most recent block that can be derived from L1 data
    pub safe_head: BlockInfo,
    /// Batch epoch of the safe head
    pub safe_epoch: Epoch,
    /// Most recent block that can be derived from finalized L1 data
    pub finalized_head: BlockInfo,
    /// Batch epoch of the finalized head
    pub finalized_epoch: Epoch,
}

impl EngineDriver {
    /// Create a new [EngineDriver][crate::engine::EngineDriver] instance.
    pub fn new(
        finalized_head: BlockInfo,
        finalized_epoch: Epoch,
        provider: MockProvider,
        config: impl Into<EngineConfig>,
    ) -> Result<Self> {
        let engine_config: EngineConfig = config.into();
        let blocktime = engine_config.blocktime;
        let engine = EngineApi::from(engine_config);
        Ok(Self {
            engine,
            provider,
            blocktime,
            unsafe_head: finalized_head,
            safe_head: finalized_head,
            safe_epoch: finalized_epoch,
            finalized_head,
            finalized_epoch,
        })
    }
}

// impl EngineDriver {
//     pub fn handle_attributes(&mut self, attributes: PayloadAttributes) -> Result<()> {
//         let block: Option<Block<Transaction>> = self.block_at(attributes.timestamp.as_u64()).await;
//
//         if let Some(block) = block {
//             if should_skip(&block, &attributes)? {
//                 self.skip_attributes(attributes, block).await
//             } else {
//                 self.unsafe_head = self.safe_head;
//                 self.process_attributes(attributes).await
//             }
//         } else {
//             self.process_attributes(attributes).await
//         }
//     }
// }
