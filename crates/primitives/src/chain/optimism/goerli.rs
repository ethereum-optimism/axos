use alloy_primitives::{address, b256, U256};

use crate::BlockInfo;
use crate::ChainConfig;
use crate::Epoch;
use crate::GenericString;
use crate::SystemConfig;

impl ChainConfig {
    /// Optimism Goerli [ChainConfig].
    pub fn optimism_goerli() -> Self {
        Self {
            network: GenericString::from("optimism-goerli"),
            l1_chain_id: 5,
            l2_chain_id: 420,
            l1_start_epoch: Epoch {
                hash: b256!("6ffc1bf3754c01f6bb9fe057c1578b87a8571ce2e9be5ca14bace6eccfd336c7"),
                number: 8300214,
                timestamp: 1673550516,
            },
            l2_genesis: BlockInfo {
                hash: b256!("0f783549ea4313b784eadd9b8e8a69913b368b7366363ea814d7707ac505175f"),
                number: 4061224,
                parent_hash: b256!(
                    "31267a44f1422f4cab59b076548c075e79bd59e691a23fbce027f572a2a49dc9"
                ),
                timestamp: 1673550516,
            },
            system_config: SystemConfig {
                batch_sender: address!("7431310e026b69bfc676c0013e12a1a11411eec9"),
                gas_limit: U256::from(25_000_000),
                l1_fee_overhead: U256::from(2100),
                l1_fee_scalar: U256::from(1000000),
                unsafe_block_signer: address!("715b7219D986641DF9eFd9C7Ef01218D528e19ec"),
            },
            system_config_contract: address!("Ae851f927Ee40dE99aaBb7461C00f9622ab91d60"),
            batch_inbox: address!("ff00000000000000000000000000000000000420"),
            deposit_contract: address!("5b47E1A08Ea6d985D6649300584e6722Ec4B1383"),
            l2_to_l1_message_passer: address!("EF2ec5A5465f075E010BE70966a8667c94BCe15a"),
            max_channel_size: 100_000_000,
            channel_timeout: 300,
            seq_window_size: 3600,
            max_seq_drift: 600,
            regolith_time: 1679079600,
            blocktime: 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const OPTIMISM_GOERLI: &str = r#"
        {
            "network": "optimism-goerli",
            "l1_chain_id": 5,
            "l2_chain_id": 420,
            "l1_start_epoch": {
                "number": 8300214,
                "hash": "6ffc1bf3754c01f6bb9fe057c1578b87a8571ce2e9be5ca14bace6eccfd336c7",
                "timestamp": 1673550516
            },
            "l2_genesis": {
                "hash": "0f783549ea4313b784eadd9b8e8a69913b368b7366363ea814d7707ac505175f",
                "number": 4061224,
                "parent_hash": "31267a44f1422f4cab59b076548c075e79bd59e691a23fbce027f572a2a49dc9",
                "timestamp": 1673550516
            },
            "system_config": {
                "batch_sender": "7431310e026b69bfc676c0013e12a1a11411eec9",
                "gas_limit": "25000000",
                "l1_fee_overhead": "2100",
                "l1_fee_scalar": "1000000",
                "unsafe_block_signer": "715b7219D986641DF9eFd9C7Ef01218D528e19ec"
            },
            "system_config_contract": "Ae851f927Ee40dE99aaBb7461C00f9622ab91d60",
            "batch_inbox": "ff00000000000000000000000000000000000420",
            "deposit_contract": "5b47E1A08Ea6d985D6649300584e6722Ec4B1383",
            "l2_to_l1_message_passer": "EF2ec5A5465f075E010BE70966a8667c94BCe15a",
            "max_channel_size": 100000000,
            "channel_timeout": 300,
            "seq_window_size": 3600,
            "max_seq_drift": 600,
            "regolith_time": 1679079600,
            "blocktime": 2
        }
    "#;

    #[test]
    #[cfg(feature = "serde_json")]
    fn test_optimism_goerli() {
        let config = ChainConfig::optimism_goerli();
        let parsed = serde_json::from_str::<ChainConfig>(OPTIMISM_GOERLI).unwrap();
        assert_eq!(config, parsed);
    }
}
