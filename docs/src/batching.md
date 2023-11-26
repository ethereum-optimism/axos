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

Phew, now that the core mechanics of the batch submission
pipeline are covered, the next sections of this
[batching](#batching) chapter will review batch submission
through a visual representation and then dive into the
wire format used in channel's streaming compression and
batch/frame encoding.

### Wire Format

<img src="https://raw.githubusercontent.com/refcell/axos/main/docs/assets/batch-deriv-chain.svg" style="border-radius: 20px">

At the top of the diagram are L1 blocks with their
corresponding block numbers. Notice time moves to the right
throughout the diagram. Underneath the L1 blocks are the
batcher transactions included in the block. The squiggles
represent deposit transactions and the colored boxes are
[**channel frames**][cf]. So `A` and `B` denote channels
and `A0`, `A1`, `B0`, `B1`, `B2` are frames.

Importantly, the overlapping of frames from different
channels demonstrate that batcher transactions can be
interleaved. Take OP Mainnet for example, with roughly
6 L2 blocks per L1 block, there are 6 sequencer batches
that need to be compressed into channels, the frames, and
subsequently posted as calldata in batcher transactions.
It's likely that multiple batcher transactions are confirmed
per L1 block, even if there are multiple frames in a given
transaction.

Furthermore, frames do not need to be transmitted in order.

Since channel B is seen first, it is decoded into the round
boxes representing **sequencer batches** below first, then
channel A is decoded. In practice, this need not be true.
Blocks may be [_eagerly derived_][ed] but this will be
covered in the derivation spec.

The bottom half of the image demonstrates the derivation
pipeline counterpart to batch submission - deriving the
cannonical L2 Blocks from the batch submitted, compressed
calldata on the data availability layer.

There are a few key conceptual parts required for derivation
that are abstracted away from the batcher service. First,
each L2 block contains a `L1BlockInfo` that contains info
about the origin L1 block with the "sequence number" helping
to distinguish between blocks with the same L1 block origin.

The origin L1 Block info is written as the **epoch** number
which corresponds to an L1 block number. The sequencer number
is the position of the sequencer batch within the epoch.

### Batcher Transaction Format

Transactions sent by the batcher are encoded in the
following format `version_byte . rollup_payload`, where
`.` is a concatenation.

Currently `0` is the only version used where the
`rollup_payload` is a sequential concatenation of
frame data.

Unknown versions or invalid decodings will cause the
batch to fail to be included in the canonical rollup.

#### Batcher Transaction Authentication

The batcher service sends transactions with the
`to` destination address set to the **batch inbox
address**. Chain derivation checks all transactions
that are sent to the batch inbox address.

Since any actor can send a transaction to the
batch inbox address, the `from` address must match
the **batch sender address** specified in the
[SystemConfig][sc] at the time of the L1 Block that
the transaction is sent from.

### Channel Frame Format

```
frame = channel_id . frame_number . frame_data_length . frame_data . is_last

channel_id        = bytes16
frame_number      = uint16
frame_data_length = uint32
frame_data        = bytes
is_last           = bool
```

All data in a frame is fixed-size, except the `frame_data`.

The fixed overhead is `16` + `2` + `4` + `1` = `23` bytes.

Fixed-size frame metadata avoids a circular dependency with the
target total data length, to simplify packing of frames with
varying content length.

`channel_id`: A unique channel identifier.
`frame_number`: The position of the frame inside the channel.
`frame_data_length`: The number of `frame_data` bytes. The maximum
is `1_000_000` bytes.
`frame_data`: Concatenated frame data bytes.
`is_last`: Single byte where `1` indicates this frame is the last in
the channel, and `0` indicating otherwise.

### Channel Encoding

```
rlp_batches = []
for batch in batches:
    rlp_batches.append(batch)
channel_encoding = compress(rlp_batches)
```
Where:
`batches`: Sequence of [byte-encoded batches](#batch-encoding).
`rlp_batches`: Concatenation of RLP-encoded batches.
`compress`: Compression function. ([RFC-1950][1950] ZLIB function)
`channel_encoding`: Compressed version of `rlp_batches`.

Decoding the compressed channel data is limited to `MAX_RLP_BYTES_PER_CHANNEL`.
This is currently set to `10_000_000` bytes.

Decoded data exceeding the `MAX_RLP_BYTES_PER_CHANNEL` is truncated
and does not break the channel decoding.

Using the compression scheme from [RFC-1950][1950], channels
may be decoded in _streaming_ fashion where not all batches are
known in advance. On the batcher side, this allows batcher transactions
to be submitted without knowing how many batches, and frames, the
channel will contain.

### Batch Format

The sequencer batch is a list of the L2 block transactions.

This is encoded as `batch_version . content`.

Batcher versions and corresponding `content`:
`0`: `rlp_encode([parent_hash, epoch_number, epoch_hash, timestamp, transaction_list])`

Where:
`parent_hash`: The previous L2 block hash.
`epoch_number`: L1 block number for the [sequencing epoch][se].
`epoch_hash`: L1 block hash for the [sequencing epoch][se].
`timestamp`: The timestamp of the **L2 block**.
`transaction_list`: rlp-encoded list of [eip-2718][2718] transactions.

The `rlp_encode` function follows the [RLP Format][rlp].

Batches with unknown batch versions are invalid and must be ignored
in the derivation pipeline. If batches have malformed contents, they
are also ignored.

`epoch_number` and `timestamp` values must follow the contraints
outlined in the [derivation][d] specs, or the batch is considered
invalid.

<!-- Intradoc refs -->

[d]: ./derivation.md
[sc]: ./glossary.md#system-config 
[cf]: ./glossary.md#channel-frames
[ed]: ./derivation.md#eager-derivation

<!-- Hyperlinks -->

[2718]: https://eips.ethereum.org/EIPS/eip-2718
[rlp]: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp
[1950]: https://www.rfc-editor.org/rfc/rfc1950.html
[c]: https://github.com/ethereum-optimism/optimism/blob/develop/op-batcher/batcher/channel.go#L17
[cm]: https://github.com/ethereum-optimism/optimism/blob/develop/op-batcher/batcher/channel_manager.go#L27
[ee]: https://help.optimism.io/hc/en-us/articles/8004922894491-What-is-the-difference-between-EVM-equivalence-and-Ethereum-equivalence
[bu]: https://blog.oplabs.co/reproduce-bedrock-migration/#:~:text=On%20June%206%2C%202023%2C%20OP,to%20be%20verifiable%20and%20reproducible.
[s]: https://medium.com/@richardchen_81235/intro-to-shared-sequencing-1622d1fd51c9
[b]: https://github.com/ethereum-optimism/optimism/tree/develop/op-batcher
[brsd]: https://github.com/ethereum-optimism/optimism/blob/develop/specs/batcher.md?plain=1
[7f8b74d]: https://github.com/ethereum-optimism/optimism/commit/7f8b74de271069f16f7553c8e3f698e7ba61505a
