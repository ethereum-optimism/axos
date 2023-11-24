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

## The Sequencer

The sequencer is the set of services that is responsible for
accepting L2 or rollup transactions and posting the corresponding
batch submission data to the rollups data availability layer.

The current sequencer is naive and centralized. Core devs and
OP Collective contributors are hard at work to make the sequencer
[shared][s] so that a sequencer can sequence rollup transactions
across multiple OP Stack chains participating in the superchain
and decentralized sequencing. The first step for decentralized
sequencing is leader election which will likely still require
the set of sequencers or "leaders" to be permissioned.

Back to the current implementation details, the OP Mainnet 
sequencer role is illustrated in the below wireframe.

<img src="https://raw.githubusercontent.com/refcekk/axos/main/etc/sequencer.png" style="border-radius: 20px">

## Batcher

The batch submitter or [batcher][b] is the service or component
of the sequencer that submits L2 transaction data to the data
availability layer in a [compressed wire format](#wire-format).

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

It's important to realize that the L2 view of the safe/unsafe head
is not updated instantaneously after the L1 transactions are
submitted. As a result, the batcher must carefully track which
data has already been submitted and which frames/channels/l2 unsafe
transactions did not fit in the sequencing window, were dropped, or
otherwise not submitted to the data availability layer.

A frequently recurring question regarding batch submission is
how the subsequent derivation pipeline knows which data is canonical.
For example, what if an unpermissioned actor sends a transaction
on the data availability layer to the batch inbox address where
batch transactions are sent? Well the derivation pipeline checks
the transaction sender is the canonical batch submitter, or sequencer.
In the future, once leader election and decentralized sequencing
are implemented, this sender address will change based on the
updated batch submission and derivation rules.

The [derivation][d] specs will go over this in further detail.

## Bedrock

Backpedaling a bit, the OP Collective successfully completed the
[Bedrock Upgrade][bu] on OP Mainnet in 2023, to mark the first
official release of the OP Stack, making OP Stack chains
as close as possible to [Ethereum-equivalence][ee].

This upgrade introduced fixed block times of `2s`. Since the
data availability layer is Ethereum which has a fixed block time
of 12s post-merge, it is expected that each epoch will contain
12/2 = 6 L2 blocks on average.




## Sequencing Window

Earlier, we mentioned that transactions posted by the batcher must
be posted within the sequencing window.



## Channels

## Frames

## Wire Format

<!-- Intradoc and Hyper Links -->

[ee]: https://help.optimism.io/hc/en-us/articles/8004922894491-What-is-the-difference-between-EVM-equivalence-and-Ethereum-equivalence
[bu]: https://blog.oplabs.co/reproduce-bedrock-migration/#:~:text=On%20June%206%2C%202023%2C%20OP,to%20be%20verifiable%20and%20reproducible.
[d]: ./derivation.md
[s]: https://medium.com/@richardchen_81235/intro-to-shared-sequencing-1622d1fd51c9
[b]: https://github.com/ethereum-optimism/optimism/tree/develop/op-batcher
[brsd]: https://github.com/ethereum-optimism/optimism/blob/develop/specs/batcher.md?plain=1
[7f8b74d]: https://github.com/ethereum-optimism/optimism/commit/7f8b74de271069f16f7553c8e3f698e7ba61505a
