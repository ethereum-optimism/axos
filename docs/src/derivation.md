# Derivation

_Before diving into chain derivation below, make sure you have
a strong fundamental understanding of [batch submission][b]._ 

The derivation pipeline acts as the counterpart to [batch submission][b].

Where L2 transactions are _posted_ to the data availability layer,
or L1 Ethereum Mainnet for OP Mainnet, derivation is the process of
translating the batch-submitted output data on L1 into a canonical
ordering of L2 transactions.

## Genesis

Rollups start at genesis. For the sake of keeping scope limited
to the OP Stack, genesis refers to the bedrock activation block,
which for new OP Stack chains is equivalent to block 0 since
bedrock is the first official release of the OP Stack.

_Aside: visit [optimism.io][opio] for a detailed walkthrough of
the differences between OP Mainnet pre and post-bedrock._


## Derivation

The derivation pipeline is one of the key components of a [rollup][rollup].

At a high level, the derivation pipeline reads blocks from the consensus layer
(for example Ethereum Mainnet, or "L1"), and outputs canonical rollup blocks
(aka "L2" blocks). Each L1 block maps to a set of L2 blocks, called the
[sequencing epoch][se]. Since this set of L2 blocks are mapped to a single
epoch, the associated L1 block number is used as an identifier called the
[epoch number][en].


Since the size of transaction batches cannot be compressed entirely to fit
in a single block, batches are split up such that a [sequencing window][sw]
is defined that spans multiple L1 blocks.

[en]
[sw]
[se]: ./glossary.md#sequencing-epoch



### Sequencing Window

Earlier, we mentioned that transactions posted by the batcher must
be posted within the sequencing window.




<!-- Intradoc and Hyper Links -->

[b]: ./batching.md
[opio]: https://community.optimism.io/docs/developers/bedrock/explainer/#
