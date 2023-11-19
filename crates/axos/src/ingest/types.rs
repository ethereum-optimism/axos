//! Common Ingestor Types

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use axos_primitives::BlockWithTransactions;

/// Block Update
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockUpdate {
    /// A new block extending the current chain
    NewBlock(BlockWithTransactions),
    /// Updates the most recent finalized block
    FinalityUpdate(u64),
    /// Reorg detected
    Reorg,
}
