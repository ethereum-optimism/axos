use crate::alloc::string::ToString;
use alloy_primitives::{address, b256, B256, U256};

use crate::BlockInfo;
use crate::ChainConfig;
use crate::Epoch;
use crate::SystemConfig;

impl ChainConfig {
    /// Optimism Sepolia [ChainConfig].
    pub fn optimism_sepolia() -> Self {
        Self {
            network: "optimism-sepolia".to_string(),
            l1_chain_id: 11155111,
            l2_chain_id: 11155420,
            l1_start_epoch: Epoch {
                hash: b256!("48f520cf4ddaf34c8336e6e490632ea3cf1e5e93b0b2bc6e917557e31845371b"),
                number: 4071408,
                timestamp: 1691802540,
            },
            l2_genesis: BlockInfo {
                hash: b256!("102de6ffb001480cc9b8b548fd05c34cd4f46ae4aa91759393db90ea0409887d"),
                number: 0,
                parent_hash: B256::ZERO,
                timestamp: 1691802540,
            },
            system_config: SystemConfig {
                batch_sender: address!("8F23BB38F531600e5d8FDDaAEC41F13FaB46E98c"),
                gas_limit: U256::from(30_000_000),
                l1_fee_overhead: U256::from(188),
                l1_fee_scalar: U256::from(684000),
                unsafe_block_signer: address!("0000000000000000000000000000000000000000"),
            },
            system_config_contract: address!("034edd2a225f7f429a63e0f1d2084b9e0a93b538"),
            batch_inbox: address!("ff00000000000000000000000000000011155420"),
            deposit_contract: address!("16fc5058f25648194471939df75cf27a2fdc48bc"),
            l2_to_l1_message_passer: address!("4200000000000000000000000000000000000016"),
            max_channel_size: 100_000_000,
            channel_timeout: 300,
            seq_window_size: 3600,
            max_seq_drift: 600,
            regolith_time: 0,
            blocktime: 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const OPTIMISM_SEPOLIA: &str = r#"
        {
            "network": "optimism-sepolia",
            "l1_chain_id": 11155111,
            "l2_chain_id": 11155420,
            "l1_start_epoch": {
                "number": 4071408,
                "hash": "48f520cf4ddaf34c8336e6e490632ea3cf1e5e93b0b2bc6e917557e31845371b",
                "timestamp": 1691802540
            },
            "l2_genesis": {
                "number": 0,
                "hash": "102de6ffb001480cc9b8b548fd05c34cd4f46ae4aa91759393db90ea0409887d",
                "parent_hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "timestamp": 1691802540
            },
            "system_config": {
                "batch_sender": "8F23BB38F531600e5d8FDDaAEC41F13FaB46E98c",
                "gas_limit": 30000000,
                "l1_fee_overhead": 188,
                "l1_fee_scalar": 684000,
                "unsafe_block_signer": "0000000000000000000000000000000000000000"
            },
            "system_config_contract": "034edd2a225f7f429a63e0f1d2084b9e0a93b538",
            "batch_inbox": "ff00000000000000000000000000000011155420",
            "deposit_contract": "16fc5058f25648194471939df75cf27a2fdc48bc",
            "l2_to_l1_message_passer": "4200000000000000000000000000000000000016",
            "max_channel_size": 100000000,
            "channel_timeout": 300,
            "seq_window_size": 3600,
            "max_seq_drift": 600,
            "regolith_time": 0,
            "blocktime": 2
        }
    "#;

    #[test]
    fn test_optimism_sepolia() {
        let config = ChainConfig::optimism_sepolia();
        let parsed = serde_json::from_str::<ChainConfig>(OPTIMISM_SEPOLIA).unwrap();
        assert_eq!(config, parsed);
    }
}
