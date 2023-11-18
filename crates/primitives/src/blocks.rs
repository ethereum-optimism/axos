#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use alloy_primitives::B256;
pub use alloy_primitives::{BlockHash, BlockNumber};

/// A Block Identifier
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockId {
    /// The block hash
    Hash(BlockHash),
    /// The block number
    Number(BlockNumber),
}

/// A Block with Transactions
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockWithTransactions {
    /// The block hash
    pub hash: B256,
    /// The block number
    pub number: u64,
    /// The parent block hash
    pub parent_hash: B256,
    /// The block timestamp
    pub timestamp: u64,
    /// The block transactions
    #[cfg(feature = "alloc")]
    pub transactions: Vec<Transaction>,
    /// The block transactions
    #[cfg(not(feature = "alloc"))]
    pub transactions: &'static [Transaction],
}

/// A Transaction
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Transaction {
    /// The transaction hash
    pub hash: B256,
    /// The transaction index
    pub index: u64,
    /// The transaction sender
    pub sender: B256,
    /// The transaction nonce
    pub nonce: u64,
    /// The transaction gas price
    pub gas_price: u64,
    /// The transaction gas limit
    pub gas_limit: u64,
    /// The transaction to address
    pub to: Option<B256>,
    /// The transaction value
    pub value: u64,
    /// The transaction data
    #[cfg(feature = "alloc")]
    pub data: Vec<u8>,
    /// The transaction data
    #[cfg(not(feature = "alloc"))]
    pub data: &'static [u8],
    /// The transaction signature
    pub signature: Option<B256>,
}

/// Block Header Info
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
