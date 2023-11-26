# Glossary

### System Config

The system config is the set of configurable rollup parameters
maintained by the [SystemConfig contract][scc]. This contract
is deployed on the consensus layer (usually, L1 Ethereum Mainnet)
and read by the rollup derivation pipeline.

System config parameterization allows keys to be frequently
rotated and external cost parameters to be adjusted without
requiring OP Stack chains to upgrade through a hardfork.

[scc]: https://github.com/ethereum-optimism/optimism/blob/develop/specs/system_config.md

### Channel Frames

A **channel frame** is a chunk of data belonging to a channel.

Batcher transactions contain one or multiple "frames".

Channels are broken up into sequential frames so that large
channels not fitting in a single batcher transaction may still
be submitted.

### Sequencing Epoch

A sequential set of L2 blocks that are derived from a set of
l1 blocks called the [sequencing window](#sequencing-window).

The sequencing epoch is identified by an [epoch number](#epoch-number).

Sequencing epochs, or "epochs", can have variable sizes,
with constraints detailed in [derivation](./derivation.md).

### Epoch Number

The epoch number is the block number of the first L1 block
in a [sequencing window](#sequencing-window).

### Sequencing Window

A sequencing window is a range of L1 blocks from which a [sequencing 
epoch](#sequencing-epoch) can be derived.

A sequencing window whose first L1 block has number `N` contains batcher 
transactions for epoch `N`. The window contains blocks [`N`, `N` + `SWS`)
where `SWS` is the **sequencer window size**.

The current default sws is **3600 blocks**.

The first block in the window defines the depositing 
transactions which determine the deposits to be included in the first 
L2 block of the epoch.
