use crate::alloc::string::ToString;
use alloy_primitives::{address, b256, B256, U256};

use crate::BlockInfo;
use crate::ChainConfig;
use crate::Epoch;
use crate::SystemConfig;

impl ChainConfig {
    /// Base Goerli [ChainConfig].
    pub fn base_goerli() -> Self {
        Self {
            network: "base-goerli".to_string(),
            l1_chain_id: 5,
            l2_chain_id: 84531,
            l1_start_epoch: Epoch {
                number: 8410981,
                hash: b256!("73d89754a1e0387b89520d989d3be9c37c1f32495a88faf1ea05c61121ab0d19"),
                timestamp: 1675193616,
            },
            l2_genesis: BlockInfo {
                hash: b256!("a3ab140f15ea7f7443a4702da64c10314eb04d488e72974e02e2d728096b4f76"),
                number: 0,
                parent_hash: B256::ZERO,
                timestamp: 1675193616,
            },
            system_config: SystemConfig {
                batch_sender: address!("2d679b567db6187c0c8323fa982cfb88b74dbcc7"),
                gas_limit: U256::from(25_000_000),
                l1_fee_overhead: U256::from(2100),
                l1_fee_scalar: U256::from(1000000),
                unsafe_block_signer: address!("32a4e99A72c11E9DD3dC159909a2D7BD86C1Bc51"),
            },
            system_config_contract: address!("b15eea247ece011c68a614e4a77ad648ff495bc1"),
            batch_inbox: address!("8453100000000000000000000000000000000000"),
            deposit_contract: address!("e93c8cd0d409341205a592f8c4ac1a5fe5585cfa"),
            l2_to_l1_message_passer: address!("4200000000000000000000000000000000000016"),
            max_channel_size: 100_000_000,
            channel_timeout: 100,
            seq_window_size: 3600,
            max_seq_drift: 600,
            regolith_time: 1683219600,
            blocktime: 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BASE_GOERLI: &str = r#"
        {
            "network": "base-goerli",
            "l1_chain_id": 5,
            "l2_chain_id": 84531,
            "l1_start_epoch": {
                "number": 8410981,
                "hash": "73d89754a1e0387b89520d989d3be9c37c1f32495a88faf1ea05c61121ab0d19",
                "timestamp": 1675193616
            },
            "l2_genesis": {
                "hash": "a3ab140f15ea7f7443a4702da64c10314eb04d488e72974e02e2d728096b4f76",
                "number": 0,
                "parent_hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "timestamp": 1675193616
            },
            "system_config": {
                "batch_sender": "2d679b567db6187c0c8323fa982cfb88b74dbcc7",
                "gas_limit": 25000000,
                "l1_fee_overhead": 2100,
                "l1_fee_scalar": 1000000,
                "unsafe_block_signer": "32a4e99A72c11E9DD3dC159909a2D7BD86C1Bc51"
            },
            "system_config_contract": "b15eea247ece011c68a614e4a77ad648ff495bc1",
            "batch_inbox": "8453100000000000000000000000000000000000",
            "deposit_contract": "e93c8cd0d409341205a592f8c4ac1a5fe5585cfa",
            "l2_to_l1_message_passer": "4200000000000000000000000000000000000016",
            "max_channel_size": 100000000,
            "channel_timeout": 100,
            "seq_window_size": 3600,
            "max_seq_drift": 600,
            "regolith_time": 1683219600,
            "blocktime": 2
        }
    "#;

    #[test]
    fn test_base_goerli() {
        let config = ChainConfig::base_goerli();
        let parsed = serde_json::from_str::<ChainConfig>(BASE_GOERLI).unwrap();
        assert_eq!(config, parsed);
    }
}
