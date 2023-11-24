# Batching

There are two primary data sources from which the chain of canonical
rollup or L2 blocks can be derived.

- Batch submitted transactions to the data availability layer.
- Deposit transactions posted to the canonical bridge on the
consensus layer.

Note, for OP Mainnet, the data availability layer is the same as the
consensus layer, Ethereum Mainnet or "L1".

## Deposits

Deposit transactions are relatively simple in the sense that the
transaction receipts are used to re-create deposit transactions
during chain derivation. There is no compression or any sharding
for deposit transaction data.

## Batch Submission

The batch submitter or [batcher][b] is a service that submits L2
transaction data to the data availability layer.

A high-level overview of the batch submitter performs the following
steps in a loop.

1. Check if the `unsafe` l2 block number > `safe` block number.
2. Open a channel if there are unsubmitted `unsafe` blocks.
3. Encode and compress `unsafe` blocks into the channel (may overflow into next loop iteration).
4. For each frame in the channel, create a transaction with the frame data, until the channel is empty.
5. Submit the transactions to L1.

_Note, this is based off the outline provided by the
[batcher reference specification documents][brsd] in the
optimism monorepo at commit [`7f8b74d`][7f8b74d]._





<!-- Intradoc and Hyper Links -->

[b]: https://github.com/ethereum-optimism/optimism/tree/develop/op-batcher
[brsd]: https://github.com/ethereum-optimism/optimism/blob/develop/specs/batcher.md?plain=1
[7f8b74d]: https://github.com/ethereum-optimism/optimism/commit/7f8b74de271069f16f7553c8e3f698e7ba61505a
