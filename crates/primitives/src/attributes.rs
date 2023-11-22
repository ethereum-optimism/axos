//! Types for Block Attributes

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use alloy_primitives::{Bytes, B256, U256};

use crate::SetL1BlockValuesCall;

/// Attributes of a block.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AttributesDepositedCall {
    /// The block number
    pub number: u64,
    /// The block timestamp
    pub timestamp: u64,
    /// The base fee
    pub basefee: U256,
    /// The block hash
    pub hash: B256,
    /// The sequence number of the block
    pub sequence_number: u64,
    /// The batcher hash of the block
    pub batcher_hash: B256,
    /// The batching fee overhead of the block
    pub fee_overhead: U256,
    /// The batching fee scalar of the block
    pub fee_scalar: U256,
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for AttributesDepositedCall {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> anyhow::Result<Self> {
        let call = SetL1BlockValuesCall::try_from(value)?;
        Ok(Self {
            number: call.get_block_number()?,
            timestamp: call.get_block_timestamp()?,
            basefee: call.get_basefee()?,
            hash: call.get_block_hash()?,
            sequence_number: call.get_sequence_number()?,
            batcher_hash: call.get_batcher_hash()?,
            fee_overhead: call.get_l1_fee_overhead()?,
            fee_scalar: call.get_l1_fee_scalar()?,
        })
    }
}

impl TryFrom<Bytes> for AttributesDepositedCall {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> anyhow::Result<Self> {
        let call = SetL1BlockValuesCall::try_from(value)?;
        Ok(Self {
            number: call.get_block_number()?,
            timestamp: call.get_block_timestamp()?,
            basefee: call.get_basefee()?,
            hash: call.get_block_hash()?,
            sequence_number: call.get_sequence_number()?,
            batcher_hash: call.get_batcher_hash()?,
            fee_overhead: call.get_l1_fee_overhead()?,
            fee_scalar: call.get_l1_fee_scalar()?,
        })
    }
}

impl TryFrom<&[u8]> for AttributesDepositedCall {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> anyhow::Result<Self> {
        let call = SetL1BlockValuesCall::try_from(value)?;
        Ok(Self {
            number: call.get_block_number()?,
            timestamp: call.get_block_timestamp()?,
            basefee: call.get_basefee()?,
            hash: call.get_block_hash()?,
            sequence_number: call.get_sequence_number()?,
            batcher_hash: call.get_batcher_hash()?,
            fee_overhead: call.get_l1_fee_overhead()?,
            fee_scalar: call.get_l1_fee_scalar()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{b256, hex};

    const TEST_CALLDATA: [u8; 260] = hex!(
        "015d8eb900000000000000000000000000000000000000000000000000000000008768240000000000000000000000000000000000000000000000000000000064443450000000000000000000000000000000000000000000000000000000000000000e0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d00000000000000000000000000000000000000000000000000000000000000050000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9000000000000000000000000000000000000000000000000000000000000083400000000000000000000000000000000000000000000000000000000000f4240"
    );
    const BLOCK_HASH: B256 =
        b256!("0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d");
    const BATCHER_HASH: B256 =
        b256!("0000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9");

    fn check_attributes(attributes: &AttributesDepositedCall) {
        assert_eq!(attributes.number, 8874020);
        assert_eq!(attributes.timestamp, 1682191440);
        assert_eq!(attributes.basefee, U256::from(14u64));
        assert_eq!(attributes.hash, BLOCK_HASH);
        assert_eq!(attributes.sequence_number, 5);
        assert_eq!(attributes.batcher_hash, BATCHER_HASH);
        assert_eq!(attributes.fee_overhead, U256::from(2100u64));
        assert_eq!(attributes.fee_scalar, U256::from(1000000u64));
    }

    #[test]
    fn test_attributes_from_byte_slice() {
        let call = AttributesDepositedCall::try_from(&TEST_CALLDATA[..]);
        let call = call.unwrap();
        check_attributes(&call);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_attributes_from_vec() {
        let call = AttributesDepositedCall::try_from(TEST_CALLDATA.to_vec());
        let call = call.unwrap();
        check_attributes(&call);
    }
}
