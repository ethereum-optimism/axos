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
impl TryFrom<[u8; 260]> for SetL1BlockValuesCall {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 260]) -> anyhow::Result<Self> {
        check_length!(value, 260, "SetL1BlockValuesCall");
        check_selector!(value, SET_L1_BLOCK_VALUES_SELECTOR, "SetL1BlockValuesCall");
        Ok(Self(value.to_vec()))
    }
}

impl SetL1BlockValuesCall {
    /// Parses the function selector from the call.
    pub fn parse_function_selector(&self) -> anyhow::Result<FixedBytes<4>> {
        let bys = extract_bytes!(self.0, 0, 4, "SetL1BlockValuesCall");
        Ok(FixedBytes::from_slice(bys))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;

    #[test]
    fn test_set_l1_block_values_call() {
        let calldata = hex!(
            "015d8eb900000000000000000000000000000000000000000000000000000000008768240000000000000000000000000000000000000000000000000000000064443450000000000000000000000000000000000000000000000000000000000000000e0444c991c5fe1d7291ff34b3f5c3b44ee861f021396d33ba3255b83df30e357d00000000000000000000000000000000000000000000000000000000000000050000000000000000000000007431310e026b69bfc676c0013e12a1a11411eec9000000000000000000000000000000000000000000000000000000000000083400000000000000000000000000000000000000000000000000000000000f4240"
        );
        let call = SetL1BlockValuesCall::try_from(calldata).unwrap();
        assert_eq!(
            call.parse_function_selector().unwrap(),
            SET_L1_BLOCK_VALUES_SELECTOR
        );
    }
}
