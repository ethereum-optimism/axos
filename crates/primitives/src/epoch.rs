//! L1 Epoch Block Info

use crate::attributes::AttributesDepositedCall;
use alloy_primitives::B256;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

impl From<&AttributesDepositedCall> for Epoch {
    fn from(call: &AttributesDepositedCall) -> Self {
        Self {
            number: call.number,
            timestamp: call.timestamp,
            hash: call.hash,
        }
    }
}
