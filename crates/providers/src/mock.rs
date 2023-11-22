//! Mock Provider

#[cfg(feature = "alloc")]
use alloc::string::String;

use core::cell::RefCell;

use crate::provider::{Error, Provider};
use axos_primitives::{BlockId, BlockWithTransactions, FixedBytes};

/// A mock provider for testing.
#[derive(Debug, Clone)]
pub struct MockProvider {
    /// The base URL.
    #[cfg(feature = "alloc")]
    base_url: String,
    #[cfg(not(feature = "alloc"))]
    base_url: &'static str,

    /// An internal block number counter.
    block_number: RefCell<u64>,
}

impl MockProvider {
    /// Create a new mock provider with the given base URL.
    pub fn new(
        #[cfg(feature = "alloc")] base_url: String,
        #[cfg(not(feature = "alloc"))] base_url: &'static str,
    ) -> Self {
        Self {
            base_url,
            block_number: RefCell::new(0),
        }
    }
}

/// A helper macro that accepts a single u8 and returns a [FixedBytes] of length 32
/// with the first 31 bytes set to 0 and the last byte set to the input.
macro_rules! zero_hash_with_suffix {
    ($x:expr) => {{
        let mut bytes = [0; 32];
        bytes[31] = $x;
        FixedBytes::from(bytes)
    }};
}

impl Provider for MockProvider {
    /// Fetch a block with transactions.
    fn get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<Option<BlockWithTransactions>, Error> {
        tracing::debug!(target: "mock_provider", "get block with txs, block id: {:?}", block_id);
        tracing::debug!(target: "mock_provider", "get block with txs, base url: {}", self.base_url);
        {
            *self.block_number.borrow_mut() += 1;
        }
        Ok(Some(BlockWithTransactions {
            number: (*self.block_number.borrow()).try_into().ok(),
            hash: zero_hash_with_suffix!(*self.block_number.borrow() as u8).into(),
            parent_hash: zero_hash_with_suffix!((*self.block_number.borrow() as u8) - 1),
            timestamp: (*self.block_number.borrow()).try_into().unwrap_or_default(),
            transactions: Default::default(),
            ..Default::default()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use crate::alloc::string::ToString;

    #[test]
    fn test_mock_provider() {
        #[cfg(feature = "alloc")]
        let base_url = "http://localhost:8080".to_string();
        #[cfg(not(feature = "alloc"))]
        let base_url = "http://localhost:8080";
        let provider = MockProvider::new(base_url);
        assert_eq!(provider.base_url, "http://localhost:8080");
    }
}
