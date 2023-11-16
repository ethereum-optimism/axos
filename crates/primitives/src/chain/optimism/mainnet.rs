use crate::alloc::string::ToString;
use alloy_primitives::{address, b256, U256};

use crate::BlockInfo;
use crate::ChainConfig;
use crate::Epoch;
use crate::SystemConfig;

impl ChainConfig {
    /// Optimism Mainnet [ChainConfig].
    pub fn optimism() -> Self {
        Self {
            network: "optimism".to_string(),
            l1_chain_id: 1,
            l2_chain_id: 10,
            l1_start_epoch: Epoch {
                hash: b256!("438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108"),
                number: 17422590,
                timestamp: 1686068903,
            },
            l2_genesis: BlockInfo {
                hash: b256!("dbf6a80fef073de06add9b0d14026d6e5a86c85f6d102c36d3d8e9cf89c2afd3"),
                number: 105235063,
                parent_hash: b256!(
                    "21a168dfa5e727926063a28ba16fd5ee84c814e847c81a699c7a0ea551e4ca50"
                ),
                timestamp: 1686068903,
            },
            system_config: SystemConfig {
                batch_sender: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
                gas_limit: U256::from(30_000_000),
                l1_fee_overhead: U256::from(188),
                l1_fee_scalar: U256::from(684000),
                unsafe_block_signer: address!("AAAA45d9549EDA09E70937013520214382Ffc4A2"),
            },
            batch_inbox: address!("ff00000000000000000000000000000000000010"),
            deposit_contract: address!("bEb5Fc579115071764c7423A4f12eDde41f106Ed"),
            system_config_contract: address!("229047fed2591dbec1eF1118d64F7aF3dB9EB290"),
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

    const OPTIMISM: &str = r#"
        {
            "network": "optimism",
            "l1_chain_id": 1,
            "l2_chain_id": 10,
            "l1_start_epoch": {
                "number": 17422590,
                "hash": "438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108",
                "timestamp": 1686068903
            },
            "l2_genesis": {
                "hash": "dbf6a80fef073de06add9b0d14026d6e5a86c85f6d102c36d3d8e9cf89c2afd3",
                "number": 105235063,
                "parent_hash": "21a168dfa5e727926063a28ba16fd5ee84c814e847c81a699c7a0ea551e4ca50",
                "timestamp": 1686068903
            },
            "system_config": {
                "batch_sender": "6887246668a3b87f54deb3b94ba47a6f63f32985",
                "gas_limit": 30000000,
                "l1_fee_overhead": 188,
                "l1_fee_scalar": 684000,
                "unsafe_block_signer": "AAAA45d9549EDA09E70937013520214382Ffc4A2"
            },
            "batch_inbox": "ff00000000000000000000000000000000000010",
            "deposit_contract": "bEb5Fc579115071764c7423A4f12eDde41f106Ed",
            "system_config_contract": "229047fed2591dbec1eF1118d64F7aF3dB9EB290",
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
    fn test_optimism() {
        let config = ChainConfig::optimism();
        let parsed = serde_json::from_str::<ChainConfig>(OPTIMISM).unwrap();
        assert_eq!(config, parsed);
    }
}
