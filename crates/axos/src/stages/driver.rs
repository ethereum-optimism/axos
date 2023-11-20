//! Derivation Pipeline Driver

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::info::HeadInfoQuery;
use crate::ingest::poll_queue::PollQueue;
use crate::ingest::BlockIngestor;
use axos_primitives::ChainConfig;
use axos_primitives::GenericString;
use axos_providers::provider::Provider;

use tracing::instrument;

/// Derivation Pipeline Driver
pub struct Driver {
    // todo: put this box behind the alloc feature
    /// The block ingestor.
    pub ingestor: Box<dyn BlockIngestor>,
}

impl core::fmt::Debug for Driver {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("Driver").finish()
    }
}

/// The driver configuration.
#[derive(Debug, Default)]
pub struct DriverConfig {
    /// The l2 rpc url.
    pub l2_rpc_url: GenericString,
    /// The chain config
    pub chain_config: ChainConfig,
}

// todo: implement from for the top-level Derivation Pipeline Config
//       (`From<Config> for DriverConfig`) and for the Driver.
//       Right now, it is just constructed manually in the `axt`
//       binary. Once the top-level Config is implemented, the
//       `axt` binary should be updated to build it instead of
//       building the DriverConfig directly, and then create
//       the Driver from the top-level Config like so:
//       `Driver::from(config)`.

impl From<DriverConfig> for Driver {
    #[instrument(skip(config))]
    fn from(config: DriverConfig) -> Self {
        tracing::info!("Building driver from config");
        let provider = axos_providers::mock::MockProvider::new(config.l2_rpc_url);
        tracing::debug!("Constructed provider");
        let head = HeadInfoQuery::get_head_info(&provider, &config.chain_config);
        tracing::debug!("Fetched head info");
        let finalized_head = head.l2_block_info;
        let finalized_epoch = head.l1_epoch;
        let finalized_seq = head.sequence_number;
        tracing::info!("Starting from finalized head: {:?}", finalized_head.hash);
        tracing::debug!("Finalized epoch: {:?}", finalized_epoch);
        tracing::debug!("Finalized sequence num: {:?}", finalized_seq);

        #[cfg(feature = "alloc")]
        let mut poll_queue = PollQueue::from(Box::new(provider) as Box<dyn Provider>);
        #[cfg(not(feature = "alloc"))]
        let mut poll_queue = PollQueue::from(&mut provider as &mut dyn BlockIngestor);

        poll_queue.l1_start_block =
            get_l1_start_block(finalized_epoch.number, config.chain_config.channel_timeout);
        poll_queue.l2_start_block = finalized_head.number;

        // TODO: engine driver

        // TODO:

        Self {
            ingestor: Box::new(poll_queue),
        }
    }
}

/// Calculates the L1 start block number.
/// If an overflow occurs during subtraction, the function returns the genesis block #0.
pub fn get_l1_start_block(epoch_number: u64, channel_timeout: u64) -> u64 {
    epoch_number.saturating_sub(channel_timeout)
}

impl Driver {
    /// Instantiates a new [Driver].
    pub fn new(ingestor: Box<dyn BlockIngestor>) -> Self {
        Self { ingestor }
    }
}
