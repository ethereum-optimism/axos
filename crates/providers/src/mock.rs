//! Mock Provider

#[cfg(feature = "alloc")]
use alloc::string::String;

use crate::provider::{Error, Provider};
use axos_primitives::{BlockId, BlockWithTransactions};

/// A mock provider for testing.
#[derive(Debug, Clone)]
pub struct MockProvider {
    /// The base URL.
    #[cfg(feature = "alloc")]
    base_url: String,
    #[cfg(not(feature = "alloc"))]
    base_url: &'static str,
}

impl MockProvider {
    /// Create a new mock provider with the given base URL.
    pub fn new(
        #[cfg(feature = "alloc")] base_url: String,
        #[cfg(not(feature = "alloc"))] base_url: &'static str,
    ) -> Self {
        Self { base_url }
    }
}

impl Provider for MockProvider {
    /// Fetch a block with transactions.
    fn get_block_with_txs(
        &self,
        _block_id: BlockId,
    ) -> Result<Option<BlockWithTransactions>, Error> {
        tracing::debug!("get_block_with_txs, base url: {}", self.base_url);
        Ok(Some(BlockWithTransactions::default()))
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
