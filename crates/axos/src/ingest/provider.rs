//! Internal Provider Wrapper Logic

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use axos_primitives::{BlockId, BlockWithTransactions};
use axos_providers::mock::MockProvider;
use axos_providers::provider::{Error, Provider};

/// InnerProvider wraps a [Provider].
#[derive(Default)]
pub struct InnerProvider(
    #[cfg(feature = "alloc")] pub Option<Box<dyn Provider>>,
    #[cfg(not(feature = "alloc"))] pub Option<&'static mut dyn Provider>,
);

#[cfg(feature = "alloc")]
impl From<MockProvider> for InnerProvider {
    fn from(provider: MockProvider) -> Self {
        Self(Some(Box::new(provider)))
    }
}

#[cfg(not(feature = "alloc"))]
impl From<MockProvider> for InnerProvider {
    fn from(provider: MockProvider) -> Self {
        Self(Some(&mut provider))
    }
}

#[cfg(feature = "alloc")]
impl From<Box<dyn Provider>> for InnerProvider {
    fn from(provider: Box<dyn Provider>) -> Self {
        Self(Some(provider))
    }
}

#[cfg(not(feature = "alloc"))]
impl From<&'static mut dyn Provider> for InnerProvider {
    fn from(provider: &'static mut dyn Provider) -> Self {
        Self(Some(provider))
    }
}

// Debug implementation for InnerProvider
impl core::fmt::Debug for InnerProvider {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            #[cfg(feature = "alloc")]
            Some(_) => write!(f, "InnerProvider(Some(Provider))"),
            #[cfg(not(feature = "alloc"))]
            Some(_) => write!(f, "InnerProvider(Some(Provider))"),
            None => write!(f, "InnerProvider(None)"),
        }
    }
}

impl Provider for InnerProvider {
    fn get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<Option<BlockWithTransactions>, Error> {
        match self.0 {
            #[cfg(feature = "alloc")]
            Some(ref provider) => provider.get_block_with_txs(block_id),
            #[cfg(not(feature = "alloc"))]
            Some(provider) => provider.get_block_with_txs(block_id),
            None => Err(Error::BlockNotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use axos_providers::mock::MockProvider;

    fn build_mock_provider() -> MockProvider {
        #[cfg(feature = "alloc")]
        let url = "http://localhost:8080".to_string();
        #[cfg(not(feature = "alloc"))]
        let url = "http://localhost:8080";
        MockProvider::new(url)
    }

    #[test]
    fn test_provider() {
        let provider = build_mock_provider();
        let inner_provider = InnerProvider::from(provider);
        assert!(inner_provider
            .get_block_with_txs(BlockId::Number(1))
            .is_ok());
        assert!(inner_provider
            .get_block_with_txs(BlockId::Number(1))
            .is_ok());
    }
}
