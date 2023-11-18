use axos_primitives::{BlockId, BlockKind, ChainConfig, HeadInfo};
use axos_providers::provider::Provider;

/// A query for the head info.
#[derive(Debug, Clone)]
pub struct HeadInfoQuery {}

impl HeadInfoQuery {
    /// Get the head info.
    pub fn get_head_info<P: Provider>(p: &P, config: &ChainConfig) -> HeadInfo {
        let b = p
            .get_block_with_txs(BlockId::Kind(BlockKind::Finalized))
            .ok()
            .flatten()
            .and_then(|block| HeadInfo::try_from(block).ok());
        b.unwrap_or_else(|| {
            tracing::warn!("could not get head info. Falling back to the genesis head.");
            HeadInfo {
                l2_block_info: config.l2_genesis,
                l1_epoch: config.l1_start_epoch,
                sequence_number: 0,
            }
        })
    }
}
