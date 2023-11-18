use alloy_primitives::{address, b256, B256, U256};

#[cfg(feature = "alloc")]
use alloc::string::ToString;

use crate::BlockInfo;
use crate::ChainConfig;
use crate::Epoch;
use crate::SystemConfig;

impl ChainConfig {
    /// Base Mainnet [ChainConfig].
    pub fn base() -> Self {
        Self {
            #[cfg(feature = "alloc")]
            network: "base".to_string(),
            #[cfg(not(feature = "alloc"))]
            network: "base",
            l1_chain_id: 1,
            l2_chain_id: 8453,
            l1_start_epoch: Epoch {
                number: 17481768,
                hash: b256!("5c13d307623a926cd31415036c8b7fa14572f9dac64528e857a470511fc30771"),
                timestamp: 1686789347,
            },
            l2_genesis: BlockInfo {
                hash: b256!("f712aa9241cc24369b143cf6dce85f0902a9731e70d66818a3a5845b296c73dd"),
                number: 0,
                parent_hash: B256::ZERO,
                timestamp: 1686789347,
            },
            system_config: SystemConfig {
                batch_sender: address!("5050f69a9786f081509234f1a7f4684b5e5b76c9"),
                gas_limit: U256::from(30000000),
                l1_fee_overhead: U256::from(188),
                l1_fee_scalar: U256::from(684000),
                unsafe_block_signer: address!("Af6E19BE0F9cE7f8afd49a1824851023A8249e8a"),
            },
            batch_inbox: address!("ff00000000000000000000000000000000008453"),
            deposit_contract: address!("49048044d57e1c92a77f79988d21fa8faf74e97e"),
            system_config_contract: address!("73a79fab69143498ed3712e519a88a918e1f4072"),
            l2_to_l1_message_passer: address!("4200000000000000000000000000000000000016"),
            max_channel_size: 100_000_000,
            channel_timeout: 300,
            seq_window_size: 3600,
            max_seq_drift: 600,
            blocktime: 2,
            regolith_time: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BASE: &str = r#"
        {
            "network": "base",
            "l1_chain_id": 1,
            "l2_chain_id": 8453,
            "l1_start_epoch": {
                "number": 17481768,
                "hash": "5c13d307623a926cd31415036c8b7fa14572f9dac64528e857a470511fc30771",
                "timestamp": 1686789347
            },
            "l2_genesis": {
                "hash": "f712aa9241cc24369b143cf6dce85f0902a9731e70d66818a3a5845b296c73dd",
                "number": 0,
                "timestamp": 1686789347,
                "parent_hash": "0000000000000000000000000000000000000000000000000000000000000000"
            },
            "system_config": {
                "batch_sender": "5050f69a9786f081509234f1a7f4684b5e5b76c9",
                "gas_limit": 30000000,
                "l1_fee_overhead": 188,
                "l1_fee_scalar": 684000,
                "unsafe_block_signer": "Af6E19BE0F9cE7f8afd49a1824851023A8249e8a"
            },
            "batch_inbox": "ff00000000000000000000000000000000008453",
            "deposit_contract": "49048044d57e1c92a77f79988d21fa8faf74e97e",
            "system_config_contract": "73a79fab69143498ed3712e519a88a918e1f4072",
            "l2_to_l1_message_passer": "4200000000000000000000000000000000000016",
            "max_channel_size": 100000000,
            "channel_timeout": 300,
            "seq_window_size": 3600,
            "max_seq_drift": 600,
            "blocktime": 2,
            "regolith_time": 0
        }
    "#;

    #[test]
    #[cfg(feature = "serde_json")]
    fn test_base_goerli() {
        let config = ChainConfig::base();
        let parsed = serde_json::from_str::<ChainConfig>(BASE).unwrap();
        assert_eq!(config, parsed);
    }
}
