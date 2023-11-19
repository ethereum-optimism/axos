//! Poll Queue
//!
//! The poll queue is a one-shot, synchronous [BlockIngestor] implementation
//! that polls the provider for new blocks and ingests them into an internal
//! queue. Seen blocks are not re-ingested.

use axos_primitives::{BlockId, BlockKind, BlockWithTransactions};
use axos_providers::provider::Provider;

use crate::ingest::*;

/// Poll Queue
#[derive(Debug, Default)]
pub struct PollQueue {
    /// The block queue.
    queue: queue::InnerQueue,

    /// An internal reference to the dynamic [axos_providers::provider::Provider] trait.
    provider: provider::InnerProvider,

    /// The L1 starting block
    pub l1_start_block: u64,
    /// The L2 starting block
    pub l2_start_block: u64,
    /// The L1 latest block
    pub l1_latest_block: u64,
    /// The L2 latest block
    pub l2_latest_block: u64,
}

impl PollQueue {
    /// Instantiates a new [PollQueue].
    #[cfg(feature = "alloc")]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Instantiates a new [PollQueue].
    #[cfg(not(feature = "alloc"))]
    pub fn new(queue: &'static mut VecDeque<BlockWithTransactions>) -> Self {
        Self {
            queue,
            ..Default::default()
        }
    }

    /// Load a specific block into the queue by block number.
    /// The block number must not already be seen by the ingestor
    /// or be in the queue.
    pub fn load_block(&mut self, block_number: u64) {
        // Check if the block is already seen
        if block_number < self.l1_start_block {
            return;
        }

        // Check if the block is already in the queue
        if self.queue.iter().any(|b| b.number == block_number) {
            return;
        }

        match self
            .provider
            .get_block_with_txs(BlockId::Number(block_number))
        {
            Ok(Some(block)) => self.queue.push_back(block),
            Ok(None) => {
                tracing::warn!(
                    "[load_block] block {} not found. Falling back to the genesis block.",
                    block_number
                );
            }
            Err(e) => {
                tracing::error!("[load_block] error fetching block: {:?}", e);
            }
        };
    }

    /// Loads blocks from the provider into the queue.
    fn load_blocks(&mut self) {
        // Get the latest known block
        let latest_block = match self
            .provider
            .get_block_with_txs(BlockId::Kind(BlockKind::Latest))
        {
            Ok(Some(block)) => block,
            Ok(None) => {
                tracing::info!("[poll] no new blocks");
                return;
            }
            Err(e) => {
                tracing::error!("[poll] error fetching latest block: {:?}", e);
                return;
            }
        };

        let blocks = self.l2_latest_block..latest_block.number;
        blocks.for_each(|block_number| {
            self.load_block(block_number);
        });
    }

    /// Retrieves the next block from the queue.
    pub fn next_block(&mut self) -> Option<BlockWithTransactions> {
        self.load_blocks();
        self.queue.pop_front()
    }
}

impl BlockIngestor for PollQueue {
    fn try_ingest(&mut self) -> anyhow::Result<Option<BlockUpdate>> {
        Ok(self.next_block().map(BlockUpdate::NewBlock))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[cfg(feature = "alloc")]
    // use alloc::vec::Vec;
    // use axos_primitives::BlockWithTransactions;

    // #[test]
    // fn test_ingestor() {
    //     let mut ingestor = Ingestor::new();
    //     let block = BlockWithTransactions::default();
    //     let block2 = ingestor.try_ingest().unwrap();
    //     assert_eq!(block, block2);
    // }
    //
    // #[test]
    // #[cfg(feature = "alloc")]
    // fn test_ingestor_queue() {
    //     let mut ingestor = Ingestor::new();
    //     let block = BlockWithTransactions::default();
    //     let block2 = ingestor.try_ingest().unwrap();
    //     assert_eq!(block, block2);
    // }
}
