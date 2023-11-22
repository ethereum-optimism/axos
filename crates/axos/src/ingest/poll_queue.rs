//! Poll Queue
//!
//! The poll queue is a one-shot, synchronous [BlockIngestor] implementation
//! that polls the provider for new blocks and ingests them into an internal
//! queue. Seen blocks are not re-ingested.
//!
//! ## Example
//!
//! The [PollQueue] implements the [Iterator] trait, so it can be used as an
//! iterator providing [BlockWithTransactions].
//!
#![cfg_attr(
    feature = "alloc",
    doc = r#"
```rust
use axos_providers::provider::Provider;
use axos::ingest::poll_queue::PollQueue;
use axos_providers::mock::MockProvider;
use axos_primitives::U64;

let provider = MockProvider::new("http://localhost:8080".to_string());
let mut poll_queue = PollQueue::from(Box::new(provider) as Box<dyn Provider>);
let block = poll_queue.next().unwrap();
assert_eq!(block.number, Some(U64::from(2)));
```
"#
)]

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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

#[cfg(feature = "alloc")]
impl From<Box<dyn Provider>> for PollQueue {
    fn from(provider: Box<dyn Provider>) -> Self {
        Self {
            provider: provider.into(),
            ..Default::default()
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl From<&'static mut dyn Provider> for PollQueue {
    fn from(provider: &'static mut dyn Provider) -> Self {
        Self {
            provider: provider.into(),
            ..Default::default()
        }
    }
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
        if self.queue.iter().any(|b| {
            b.number
                .map(|n| n.to::<u64>() == block_number)
                .unwrap_or(false)
        }) {
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
    pub fn load_blocks(&mut self) {
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

        let blocks = self.l2_latest_block
            ..(latest_block
                .number
                .map(|n| n.to::<u64>())
                .unwrap_or_default());
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

impl Iterator for PollQueue {
    type Item = BlockWithTransactions;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_block()
    }
}

impl BlockIngestor for PollQueue {
    fn try_ingest(&mut self) -> anyhow::Result<Option<BlockUpdate>> {
        Ok(self.next_block().map(BlockUpdate::NewBlock))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use axos_primitives::BlockWithTransactions;
    use axos_providers::mock::MockProvider;

    fn build_mock_provider() -> MockProvider {
        #[cfg(feature = "alloc")]
        let url = "http://localhost:8080".to_string();
        #[cfg(not(feature = "alloc"))]
        let url = "http://localhost:8080";
        MockProvider::new(url)
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_ingest_mock_provider() {
        use axos_primitives::{B256, U256, U64};

        let provider = build_mock_provider();
        let mut poll_queue = PollQueue::from(Box::new(provider) as Box<dyn Provider>);
        let block = poll_queue.try_ingest().unwrap();
        let mut expected_hash = [0; 32];
        expected_hash[31] = 2;
        let mut expected_parent_hash = [0; 32];
        expected_parent_hash[31] = 1;
        let expected_block = BlockWithTransactions {
            number: Some(U64::from(2)),
            hash: Some(B256::from(expected_hash)),
            parent_hash: B256::from(expected_parent_hash),
            timestamp: U256::from(2),
            transactions: Default::default(),
            ..Default::default()
        };
        assert_eq!(block, Some(BlockUpdate::NewBlock(expected_block)));
    }

    #[test]
    #[cfg(not(feature = "alloc"))]
    fn test_ingest_mock_provider() {
        let provider = build_mock_provider();
        let mut poll_queue = PollQueue::from(&mut provider as &mut dyn Provider);
        let block = poll_queue.try_ingest().unwrap();
        let expected_block = BlockWithTransactions {
            number: Some(U64::from(2)),
            hash: [0; 32],
            parent_hash: [0; 32],
            timestamp: U256::from(2),
            transactions: Default::default(),
            ..Default::default()
        };
        assert_eq!(block, Some(BlockUpdate::NewBlock(expected_block)));
    }
}
