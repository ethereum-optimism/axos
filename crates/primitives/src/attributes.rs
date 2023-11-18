//! Types for Block Attributes

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use alloy_primitives::{B256, U256};

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

    fn try_from(_value: &[u8]) -> anyhow::Result<Self> {
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

#[cfg(test)]
mod tests {
    use alloy_primitives::{b256, hex};

    #[test]
    fn test_manual_calldata_decode() {
        let calldata = hex!(
            "015d8eb900000000000000000000000000000000000000000000000000000000008768240000000000000000000000000000000000000000000000000000000064443450000000000000000000000000000000000000000000000000000000000000000e0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d00000000000000000000000000000000000000000000000000000000000000050000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9000000000000000000000000000000000000000000000000000000000000083400000000000000000000000000000000000000000000000000000000000f4240"
        );

        assert_eq!(calldata.len(), 260);

        // Decode the first 8 bytes as the function selector
        let selector = &calldata[0..4];
        assert_eq!(selector, hex!("015d8eb9"));

        // Decode the next 32 bytes as the block number
        let block_number = &calldata[4..36];
        assert_eq!(
            block_number,
            hex!("0000000000000000000000000000000000000000000000000000000000876824")
        );
        // Parse the lower 8 bytes as a u64 from hex
        let number = u64::from_be_bytes(block_number[24..].try_into().unwrap());
        let expected_block_number = 8874020;
        assert_eq!(expected_block_number, number);

        // Decode the next 32 bytes as the timestamp
        let timestamp = &calldata[36..68];
        assert_eq!(
            timestamp,
            hex!("0000000000000000000000000000000000000000000000000000000064443450")
        );

        // Parse the timestamp as a u64
        let time = u64::from_be_bytes(timestamp[24..].try_into().unwrap());
        let expected_timestamp = 1682191440;
        assert_eq!(expected_timestamp, time);

        // Decode the next 32 bytes as the basefee
        let basefee = &calldata[68..100];
        assert_eq!(
            basefee,
            hex!("000000000000000000000000000000000000000000000000000000000000000e")
        );

        // Decode the next 32 bytes as the expected hash
        let hash = &calldata[100..132];
        let expected_hash =
            b256!("0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d");
        assert_eq!(expected_hash, hash);

        // let call = AttributesDepositedCall::try_from(Bytes::from_str(calldata)?);
        //
        // assert!(call.is_ok());
        // let call = call.unwrap();
        //
        // assert_eq!(call.hash, expected_hash);
        // assert_eq!(call.number, expected_block_number);
        // assert_eq!(call.timestamp, expected_timestamp);
    }
}
