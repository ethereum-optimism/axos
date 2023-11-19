//! Types for Block Attributes

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use alloy_primitives::{B256, U256};

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

    fn try_from(_value: Vec<u8>) -> anyhow::Result<Self> {
        Err(anyhow::anyhow!("Not implemented"))
        // let number = u64::from_le_bytes(value[0..8].try_into()?);
        // let timestamp = u64::from_le_bytes(value[8..16].try_into()?);
        // let basefee = U256::from_little_endian(&value[16..48]);
        // let hash = B256::from_little_endian(&value[48..80]);
        // let sequence_number = u64::from_le_bytes(value[80..88].try_into()?);
        // let batcher_hash = B256::from_little_endian(&value[88..120]);
        // let fee_overhead = U256::from_little_endian(&value[120..152]);
        // let fee_scalar = U256::from_little_endian(&value[152..184]);
        //
        // Ok(Self {
        //     number,
        //     timestamp,
        //     basefee,
        //     hash,
        //     sequence_number,
        //     batcher_hash,
        //     fee_overhead,
        //     fee_scalar,
        // })
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

    #[test]
    fn test_attributes_from_byte_slice() {
        let call = AttributesDepositedCall::try_from(&TEST_CALLDATA[..]);
        let call = call.unwrap();
        let expected_hash =
            b256!("0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d");
        let batcher_hash =
            b256!("0000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9");
        assert_eq!(call.number, 8874020);
        assert_eq!(call.timestamp, 1682191440);
        assert_eq!(call.basefee, U256::from(14u64));
        assert_eq!(call.hash, expected_hash);
        assert_eq!(call.sequence_number, 5);
        assert_eq!(call.batcher_hash, batcher_hash);
        assert_eq!(call.fee_overhead, U256::from(2100u64));
        assert_eq!(call.fee_scalar, U256::from(1000000u64));
    }
}
