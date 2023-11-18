#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::attributes::AttributesDepositedCall;
use crate::blocks::{BlockInfo, BlockWithTransactions};
use crate::epoch::Epoch;

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

impl TryFrom<BlockWithTransactions> for HeadInfo {
    type Error = anyhow::Error;

    fn try_from(value: BlockWithTransactions) -> anyhow::Result<Self> {
        let tx_calldata = value
            .transactions
            .first()
            .ok_or(anyhow::anyhow!(
                "Could not find the L1 attributes deposited transaction"
            ))?
            .data
            .clone();

        let call = AttributesDepositedCall::try_from(tx_calldata)?;
        let epoch = Epoch::from(&call);

        Ok(Self {
            l2_block_info: value.try_into()?,
            l1_epoch: epoch,
            sequence_number: call.sequence_number,
        })
    }
}
