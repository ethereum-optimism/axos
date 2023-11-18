use axos_primitives::{BlockId, BlockWithTransactions};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Provider Trait
pub trait Provider {
    /// Fetch a block with transactions.
    fn get_block_with_txs(&self, block_id: BlockId)
        -> Result<Option<BlockWithTransactions>, Error>;
}

/// Provider Errors
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    /// Block not found.
    BlockNotFound,
}
