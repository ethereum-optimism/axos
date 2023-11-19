//! Chain Ingestion Service
//!
//! The chain ingestion service is responsible for ingesting blocks from the
//! consensus layer and buffering them for subsequent processing by the
//! axos derivation pipeline.

pub mod poll_queue;
pub mod provider;
pub mod queue;
pub mod types;

// Re-export common ingestor types.
#[doc(inline)]
pub use types::BlockUpdate;

/// Block Ingestor Trait
pub trait BlockIngestor {
    /// Attempt to ingest a block.
    fn try_ingest(&mut self) -> anyhow::Result<Option<BlockUpdate>>;
}
