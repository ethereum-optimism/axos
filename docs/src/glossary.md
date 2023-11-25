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

