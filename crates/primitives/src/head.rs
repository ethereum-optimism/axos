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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transaction;
    use alloy_primitives::{b256, hex, B256};

    const TEST_CALLDATA: [u8; 260] = hex!(
        "015d8eb900000000000000000000000000000000000000000000000000000000008768240000000000000000000000000000000000000000000000000000000064443450000000000000000000000000000000000000000000000000000000000000000e0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d00000000000000000000000000000000000000000000000000000000000000050000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9000000000000000000000000000000000000000000000000000000000000083400000000000000000000000000000000000000000000000000000000000f4240"
    );
    const BLOCK_HASH: B256 =
        b256!("0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d");

    #[test]
    #[cfg(feature = "alloc")]
    fn test_head_info_try_from() {
        use alloc::vec;

        let block = BlockWithTransactions {
            hash: BLOCK_HASH,
            number: 8874020,
            parent_hash: B256::default(),
            timestamp: 1682191440,
            transactions: vec![Transaction {
                hash: B256::default(),
                data: TEST_CALLDATA.to_vec(),
                ..Default::default()
            }],
        };

        let head_info = HeadInfo::try_from(block).unwrap();

        assert_eq!(head_info.l2_block_info.hash, BLOCK_HASH);
        assert_eq!(head_info.l2_block_info.number, 8874020);
        assert_eq!(head_info.l2_block_info.parent_hash, B256::default());
        assert_eq!(head_info.l2_block_info.timestamp, 1682191440);
        assert_eq!(head_info.l1_epoch.number, 8874020);
        assert_eq!(head_info.l1_epoch.hash, BLOCK_HASH);
        assert_eq!(head_info.l1_epoch.timestamp, 1682191440);
        assert_eq!(head_info.sequence_number, 5);
    }
}
