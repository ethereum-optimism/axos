# Batching

There are two primary data sources from which the chain of canonical
rollup or L2 blocks can be derived.

- Batch submitted transactions to the data availability layer.
- Deposit transactions posted to the canonical bridge on the
consensus layer.

Note, for OP Mainnet, the data availability layer is the same as the
consensus layer, Ethereum Mainnet or "L1".

### Deposits

Deposit transactions are relatively simple in the sense that the
transaction receipts are used to re-create deposit transactions
during chain derivation. There is no compression or any sharding
for deposit transaction data.

### Sequencer

The sequencer is the set of services that is responsible for
accepting L2 or rollup transactions and posting the corresponding
batch submission data to the rollups data availability layer.

The current sequencer is the naive and centralized entity.

Core OP Stack developers are hard at work to make the sequencer
[shared][s] so that a sequencer can sequence rollup transactions
across multiple OP Stack chains participating in the superchain
and decentralized sequencing. The first step for decentralized
sequencing is leader election which will likely still require
the set of sequencers or "leaders" to be permissioned.

Back to the current implementation details, the OP Mainnet 
sequencer role is illustrated in the below wireframe. By the
end of this chapter, you will understand how the different
parts of this wireframe interact in detail.

<img src="https://raw.githubusercontent.com/refcell/axos/main/docs/assets/sequencer.png" style="border-radius: 20px">

### Batcher

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

### Bedrock

Backpedaling a bit, the OP Collective successfully completed the
[Bedrock Upgrade][bu] for OP Mainnet on June 6, 2023, marking the
first official release of the OP Stack, making OP Stack chains
as close as possible to [Ethereum-equivalence][ee].

This upgrade introduced fixed block times of `2s`. Since the
data availability layer is Ethereum which has a fixed block time
of `12s` post-merge, it is expected that each epoch will contain
`12`/`2` = `6` L2 blocks on average.

Each of these L2 blocks include a state root and a reference
to the previous block. This is a key distinguishing difference
between L2 blocks and batches. Batches only commit transaction
data to the data availability layer (via calldata on Ethereum
Mainnet), associating this compressed data to a given L2
timestamp (which corresponds to a given L2 block number and
L1 block number, by extension).

Batches are thus more stateless in a sense since the output
roots that attest to the sequencer's state transition are
posted separately by the proposer service. So, as long as the
batch is encoded correctly and the transactions (signatures)
are valid, the batches posted are part of the canonical L2
chain. If the proposer service separately posts an invalid
output root, most likely from performing an invalid state
transition, it can be challenged with fault proofs.

So now we have a loose idea of how the batcher service itself
fits into batch submission, let's dive deeper into the mechanics
of how the transaction batches are split up along with their
encoding and decoding ([wire format](#wire-format)).

### Batch Submission Mechanics

At the highest level of batch submission is the **sequencer
batch**. This is a list of L2 transactions that are submitted
to the sequencer (forming the `unsafe` head) and correspond
to an epoch number (L2 timestamp / block number). In this
way, each sequencer batch corresponds to a single L2 block.

On the implementation side, the batcher service fetches the
sync status from its rollup client as illustrated in the
[sequencer](#sequencer) section. This tells the batcher how
many L2 blocks the `unsafe` head is ahead of the `safe` head.
Then, it loads all **non-batched** `unsafe` blocks into its
local state using the L2 Client component (`op-geth` or
`op-reth`).

Again, each L2 block translates to a sequencer batch.

The next layer of the batch submission pipeline is building
a **channel**. A **channel** is a streaming compression scheme
that is applied to one or multiple sequencer batches. The
reason for grouping sequencer batches together is to increase
the compression ratio. In the batcher service golang reference
implementation, this is encapsulated in the
[`channelManager`][cm]. Once the `unsafe` blocks are loaded
into the channel manager state, the batcher service asks the
channel manager to produce transaction data to post to the
data availability layer. This kicks off the compression
process and the splitting of channels into frames.

**Frames** are the next step lower in batch submission.
These are chunks of data that belong to a single channel.
Once the l2 blocks are loaded into channels, the current
channel is asked to output ready frames. This is done with
a call the the [`channel`][c] itself. The reason to chunk
a channel into groups of bytes known as frames is to ensure
frames fit inside data availability layer transactions.
This makes the batch submission process maximally flexible
over the data availability layer by simply configurin the
`MaxFrameSize` which tells the channel builder how to split
up the compressed channel bytes.

On the other hand, splitting blocks (or batches) and their
subsequent channels up into frames allows for the L2 block
size to be decoupled completely from the restrictions of
the data availability layer.

Once frames are ready, the channel manager will propagate
the transaction data from frames back up to the batcher
service for submission to the L1 Client component in
the [sequencer architecture](#sequencer). This is the
lowest level of batch submission. Frame data is posted
to the data availability layer through the L1 Client.
Again, in the case of OP Mainnet, the data availability
layer is Ethereum Mainnet.

Note, these transactions contain one _or more_ frames.

Phew, now that we've covered the core mechanics of the
batch submission pipeline, the below image should nearly
completely make sense.

<img src="https://raw.githubusercontent.com/refcell/axos/main/docs/assets/batch-deriv-chain.svg" style="border-radius: 20px">

// todo: detail image

### Sequencing Window

Earlier, we mentioned that transactions posted by the batcher must
be posted within the sequencing window.

### Wire Format

<!-- Intradoc and Hyper Links -->

[c]: https://github.com/ethereum-optimism/optimism/blob/develop/op-batcher/batcher/channel.go#L17
[cm]: https://github.com/ethereum-optimism/optimism/blob/develop/op-batcher/batcher/channel_manager.go#L27
[ee]: https://help.optimism.io/hc/en-us/articles/8004922894491-What-is-the-difference-between-EVM-equivalence-and-Ethereum-equivalence
[bu]: https://blog.oplabs.co/reproduce-bedrock-migration/#:~:text=On%20June%206%2C%202023%2C%20OP,to%20be%20verifiable%20and%20reproducible.
[d]: ./derivation.md
[s]: https://medium.com/@richardchen_81235/intro-to-shared-sequencing-1622d1fd51c9
[b]: https://github.com/ethereum-optimism/optimism/tree/develop/op-batcher
[brsd]: https://github.com/ethereum-optimism/optimism/blob/develop/specs/batcher.md?plain=1
[7f8b74d]: https://github.com/ethereum-optimism/optimism/commit/7f8b74de271069f16f7553c8e3f698e7ba61505a
