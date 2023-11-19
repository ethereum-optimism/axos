//! Internal Provider Wrapper Logic

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use axos_primitives::{BlockId, BlockWithTransactions};
use axos_providers::provider::{Error, Provider};

/// InnerProvider wraps a [Provider].
#[derive(Default)]
pub struct InnerProvider(
    #[cfg(feature = "alloc")] pub Option<Box<dyn Provider>>,
    #[cfg(not(feature = "alloc"))] pub Option<&'static mut dyn Provider>,
);

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
