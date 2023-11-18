//! Provider Trait
//!
//! This module defines the `Provider` trait, which exposes a host of
//! methods for fetching chain data.
use axos_primitives::{BlockId, BlockWithTransactions};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// todo(refcell): The provider should be removed and replaced with support
//                for alloy-providers once the alloy repository is public
//                at https://github.com/alloy-rs/alloy.

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
