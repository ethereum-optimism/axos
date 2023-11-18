#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::blocks::{BlockInfo, Epoch};

/// Block info for the current head of the chain
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HeadInfo {
    /// L2 BlockInfo value
    pub l2_block_info: BlockInfo,
    /// L1 batch epoch of the head L2 block
    pub l1_epoch: Epoch,
    /// Sequencer number of head block
    pub sequence_number: u64,
}
