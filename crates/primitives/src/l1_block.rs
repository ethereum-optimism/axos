//! Types and constants for interacting with the L1 block contract.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use alloy_primitives::fixed_bytes;
use alloy_primitives::{FixedBytes, B256, U256};

/// The `setL1BlockValues` function selector.
pub const SET_L1_BLOCK_VALUES_SELECTOR: FixedBytes<4> = fixed_bytes!("015d8eb9");

/// An input type for the `setL1BlockValues` function on the `L1Block` contract.
pub type SetL1BlockValueInput = (u64, u64, U256, B256, u64, B256, U256, U256);

/// The L1 block contract ABI.
pub const L1_BLOCK_CONTRACT_ABI: &str = r#"[
    function setL1BlockValues(uint64 _number,uint64 _timestamp, uint256 _basefee, bytes32 _hash,uint64 _sequenceNumber,bytes32 _batcherHash,uint256 _l1FeeOverhead,uint256 _l1FeeScalar) external
]"#;

/// An L1 Block Contract Call to set the L1 block values.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetL1BlockValuesCall(
    #[cfg(not(feature = "alloc"))] pub &'static [u8],
    #[cfg(feature = "alloc")] pub Vec<u8>,
);

impl SetL1BlockValuesCall {
    /// Returns the length of the calldata.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the calldata is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Helper macro that checks the length of a byte slice and
/// returns an error if it is not the expected length.
macro_rules! check_length {
    ($slice:expr, $len:expr, $func:expr) => {
        if $slice.len() != $len {
            return Err(anyhow::anyhow!("Invalid input length for {}", $func));
        }
    };
}

/// Helper macro that validates the function selector of a byte
/// slice and returns an error if it is not the expected selector.
macro_rules! check_selector {
    ($slice:expr, $selector:expr, $func:expr) => {
        let selector = FixedBytes::from_slice(&$slice[0..4]);
        if selector != $selector {
            return Err(anyhow::anyhow!("Invalid function selector for {}", $func));
        }
    };
}

/// Helper macro that extracts the bytes from a byte slice
/// giving the start and end indices. Returns an error if
/// the slice is not a superset of the specified length.
macro_rules! extract_bytes {
    ($slice:expr, $start:expr, $end:expr, $func:expr) => {{
        if $slice.len() < $end {
            return Err(anyhow::anyhow!("Invalid input length for {}", $func));
        }
        &$slice[$start..$end]
    }};
}

// Converts an input byte vector into a SetL1BlockValuesCall.
// The length of the input vector must be 260 bytes.
// The first 4 bytes must match the function selector for setL1BlockValues.
#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for SetL1BlockValuesCall {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> anyhow::Result<Self> {
        check_length!(value, 260, "SetL1BlockValuesCall");
        check_selector!(value, SET_L1_BLOCK_VALUES_SELECTOR, "SetL1BlockValuesCall");
        Ok(Self(value))
    }
}

// Converts a byte slice into a SetL1BlockValuesCall.
// The length of the input slice must be 260 bytes.
// The first 4 bytes must match the function selector for setL1BlockValues.
impl TryFrom<&[u8]> for SetL1BlockValuesCall {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> anyhow::Result<Self> {
        check_length!(value, 260, "SetL1BlockValuesCall");
        check_selector!(value, SET_L1_BLOCK_VALUES_SELECTOR, "SetL1BlockValuesCall");
        Ok(Self(value.to_vec()))
    }
}

// Converts a byte array into a SetL1BlockValuesCall.
// The length of the input slice must be 260 bytes.
// The first 4 bytes must match the function selector for setL1BlockValues.
impl TryFrom<[u8; 260]> for SetL1BlockValuesCall {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 260]) -> anyhow::Result<Self> {
        check_length!(value, 260, "SetL1BlockValuesCall");
        check_selector!(value, SET_L1_BLOCK_VALUES_SELECTOR, "SetL1BlockValuesCall");
        Ok(Self(value.to_vec()))
    }
}

impl SetL1BlockValuesCall {
    /// Parses and returns the function selector from the call.
    pub fn get_function_selector(&self) -> anyhow::Result<FixedBytes<4>> {
        let bys = extract_bytes!(self.0, 0, 4, "SetL1BlockValuesCall");
        Ok(FixedBytes::from_slice(bys))
    }

    /// Parses and returns the block number from the calldata.
    pub fn get_block_number(&self) -> anyhow::Result<u64> {
        let bys = extract_bytes!(self.0, 4, 36, "SetL1BlockValuesCall");
        let bys = bys[24..]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid input length for SetL1BlockValuesCall"))?;
        Ok(u64::from_be_bytes(bys))
    }

    /// Parses and returns the block timestamp from the calldata.
    pub fn get_block_timestamp(&self) -> anyhow::Result<u64> {
        let bys = extract_bytes!(self.0, 36, 68, "SetL1BlockValuesCall");
        let bys = bys[24..]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid input length for SetL1BlockValuesCall"))?;
        Ok(u64::from_be_bytes(bys))
    }

    /// Parses and returns the basefee from the calldata.
    pub fn get_basefee(&self) -> anyhow::Result<U256> {
        let bys = extract_bytes!(self.0, 68, 100, "SetL1BlockValuesCall");
        Ok(U256::from_be_slice(bys))
    }

    /// Parses and returns the expected hash of the block from the calldata.
    pub fn get_block_hash(&self) -> anyhow::Result<B256> {
        let bys = extract_bytes!(self.0, 100, 132, "SetL1BlockValuesCall");
        Ok(B256::from_slice(bys))
    }

    /// Parses and returns the sequence number from the calldata.
    pub fn get_sequence_number(&self) -> anyhow::Result<u64> {
        let bys = extract_bytes!(self.0, 132, 164, "SetL1BlockValuesCall");
        let bys = bys[24..]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid input length for SetL1BlockValuesCall"))?;
        Ok(u64::from_be_bytes(bys))
    }

    /// Parses and returns the batcher hash from the calldata.
    pub fn get_batcher_hash(&self) -> anyhow::Result<B256> {
        let bys = extract_bytes!(self.0, 164, 196, "SetL1BlockValuesCall");
        Ok(B256::from_slice(bys))
    }

    /// Parses and returns the L1 fee overhead from the calldata.
    pub fn get_l1_fee_overhead(&self) -> anyhow::Result<U256> {
        let bys = extract_bytes!(self.0, 196, 228, "SetL1BlockValuesCall");
        Ok(U256::from_be_slice(bys))
    }

    /// Parses and returns the L1 fee scalar from the calldata.
    pub fn get_l1_fee_scalar(&self) -> anyhow::Result<U256> {
        let bys = extract_bytes!(self.0, 228, 260, "SetL1BlockValuesCall");
        Ok(U256::from_be_slice(bys))
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
    #[cfg(feature = "alloc")]
    fn test_try_from_vec() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA.to_vec()).unwrap();
        assert_eq!(call.len(), 260);
    }

    #[test]
    fn test_get_function_selector() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let selector = call.get_function_selector().unwrap();
        assert_eq!(selector, SET_L1_BLOCK_VALUES_SELECTOR);
    }

    #[test]
    fn test_get_block_number() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let block_number = call.get_block_number().unwrap();
        assert_eq!(block_number, 8874020);
    }

    #[test]
    fn test_get_block_timestamp() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let block_timestamp = call.get_block_timestamp().unwrap();
        assert_eq!(block_timestamp, 1682191440);
    }

    #[test]
    fn test_get_basefee() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let basefee = call.get_basefee().unwrap();
        let expected_basefee = U256::from(14u64);
        assert_eq!(expected_basefee, basefee);
    }

    #[test]
    fn test_get_block_hash() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let block_hash = call.get_block_hash().unwrap();
        let expected_hash =
            b256!("0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d");
        assert_eq!(expected_hash, block_hash);
    }

    #[test]
    fn test_get_sequence_number() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let sequence_number = call.get_sequence_number().unwrap();
        assert_eq!(sequence_number, 5);
    }

    #[test]
    fn test_get_batcher_hash() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let batcher_hash = call.get_batcher_hash().unwrap();
        let expected_hash =
            b256!("0000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9");
        assert_eq!(expected_hash, batcher_hash);
    }

    #[test]
    fn test_get_l1_fee_overhead() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let l1_fee_overhead = call.get_l1_fee_overhead().unwrap();
        let expected_l1_fee_overhead = U256::from(2100u64);
        assert_eq!(expected_l1_fee_overhead, l1_fee_overhead);
    }

    #[test]
    fn test_get_l1_fee_scalar() {
        let call = SetL1BlockValuesCall::try_from(TEST_CALLDATA).unwrap();
        let l1_fee_scalar = call.get_l1_fee_scalar().unwrap();
        let expected_l1_fee_scalar = U256::from(1000000u64);
        assert_eq!(expected_l1_fee_scalar, l1_fee_scalar);
    }
}
