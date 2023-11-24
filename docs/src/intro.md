# Axos Book

_Documentation for Axos contributors and developers building ontop._ 

[![Telegram Chat][tg-badge]][tg-url]

Axos is a **portable no-std Derivation Pipeline engineered in pure Rust.** 

# Introduction

Axos is a ground-up rewrite of the [OP Stack derivation pipeline][odp]
in rust with `no_std` support.

It is built to be a portable library and modular such that parts of the
pipeline are easily interchangeable. By making pipeline inputs and outputs
minimal, concrete data types, Axos need not know anything about it's
interfaced components.

In fact, the Axos derivation pipeline is built with inspiration from
[revm][revm], using a **push** over pull architecture. This allows Axos
to know nothing about _how_ data is fetched, and instead leaves this
design decision, including coloring (async/blocking calls), up to its
downstream usage.

If you're looking to build on top of axos, using it as a library, it's
recommended that you gain a deep understanding of how [batching][b] and
[derivation][d] work and how Axos is [architected][a]. If you're familiar
or otherwise looking to jump ahead, the [Usage][u] page contains detailed
instructions for working with Axos. In addition, [Examples][e] contains
verbose examples of axos' external-facing api.

If you're looking to contribute to Axos and already have a fundamental
understanding of the OP Stack derivation and batch submission processes,
check out the [contributing][c] guide.

It's highly recommended that if you need to brush up your understanding of
batch submission and/or derivation, you read _both_ the [batching][b] and
[derivation][d] documents. These act as a living specification for batch
submission and chain derivation, aiming to be up-to-date with the
[reference specification documents][rsd].

[u]: ./usage.md
[e]: ./examples.md
[b]: ./batching.md
[d]: ./derivation.md
[a]: ./architecture.md
[c]: ./contributing.md

[odp]: https://github.com/ethereum-optimism/optimism/tree/develop/op-node/rollup/derive
[rsd]: https://github.com/ethereum-optimism/optimism/blob/develop/specs/derivation.md
[rollup]: https://community.optimism.io/docs/protocol/2-rollup-protocol/

[axos]: ../../crates/axos
[revm]: https://github.com/bluealloy/revm

[tg-url]: https://t.me/+XR8_p3qjzoFiMjEx
[tg-badge]: https://img.shields.io/badge/chat-telegram-neon
