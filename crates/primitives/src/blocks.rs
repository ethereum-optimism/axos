use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

/// Block Header Info
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockInfo {
    /// The block hash
    pub hash: B256,
    /// The block number
    pub number: u64,
    /// The parent block hash
    pub parent_hash: B256,
    /// The block timestamp
    pub timestamp: u64,
}

impl BlockInfo {
    /// Instantiates a new [BlockInfo].
    pub fn new(hash: B256, number: u64, parent_hash: B256, timestamp: u64) -> Self {
        Self {
            hash,
            number,
            parent_hash,
            timestamp,
        }
    }
}

/// L1 epoch block
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Epoch {
    /// The block number
    pub number: u64,
    /// The block hash
    pub hash: B256,
    /// The block timestamp
    pub timestamp: u64,
}

impl Epoch {
    /// Create a new [Epoch].
    pub fn new(number: u64, hash: B256, timestamp: u64) -> Self {
        Self {
            number,
            hash,
            timestamp,
        }
    }
}
