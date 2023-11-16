# axos-primitives

Primitive Optimism types in Rust.

## Types

- Chain configurations for superchains.
- Block info types for deposits and L1 Block Info.

## Example

Type usage is straighforward with verbose documentation where needed.
Axos primitives uses [alloy-primitives][alloy] under the hood, and
re-exports all used types.

[alloy]: https://crates.io/crates/alloy-primitives

```rust
use axos_primitives::{address, b256, U256, ChainConfig, SystemConfig, BlockInfo, Epoch};

// Create an Optimism Mainnet Chain Config.
let chain_config = ChainConfig::optimism();

// Create the Optimism Mainnet L1 start epoch block.
let l1_epoch = Epoch::new(
    17422590,
    b256!("438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108"),
    1686068903
);
assert_eq!(chain_config.l1_start_epoch, l1_epoch);

// Create the Optimism Mainnet L2 genesis block info,
// without the convenience [BlockInfo::new] constructor.
let l2_genesis = BlockInfo {
    hash: b256!("dbf6a80fef073de06add9b0d14026d6e5a86c85f6d102c36d3d8e9cf89c2afd3"),
    number: 105235063,
    parent_hash: b256!("21a168dfa5e727926063a28ba16fd5ee84c814e847c81a699c7a0ea551e4ca50"),
    timestamp: 1686068903,
};
assert_eq!(chain_config.l2_genesis, l2_genesis);

// Create the Optimism Mainnet System Config object
let system_config = SystemConfig {
    batch_sender: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
    gas_limit: U256::from(30_000_000),
    l1_fee_overhead: U256::from(188),
    l1_fee_scalar: U256::from(684000),
    unsafe_block_signer: address!("AAAA45d9549EDA09E70937013520214382Ffc4A2"),
};
assert_eq!(chain_config.system_config, system_config);
```
