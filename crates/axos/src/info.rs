use axos_primitives::{BlockId, BlockNumber, ChainConfig, HeadInfo};
use axos_providers::provider::Provider;

pub struct HeadInfoQuery {}

impl HeadInfoQuery {
    pub fn get_head_info<P: Provider>(p: &P, config: &ChainConfig) -> HeadInfo {
        // p.get_block_with_txs(BlockId::Number(BlockNumber::Finalized))
        //     .await
        //     .ok()
        //     .flatten()
        //     .and_then(|block| HeadInfo::try_from(block).ok())
        //     .unwrap_or_else(|| {
        //         tracing::warn!("could not get head info. Falling back to the genesis head.");
        //         HeadInfo {
        //             l2_block_info: config.l2_genesis,
        //             l1_epoch: config.l1_start_epoch,
        //             sequence_number: 0,
        //         }
        //     })
        HeadInfo {
            l2_block_info: config.l2_genesis,
            l1_epoch: config.l1_start_epoch,
            sequence_number: 0,
        }
    }
}
