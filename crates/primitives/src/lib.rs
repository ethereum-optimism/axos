#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Re-export used [alloy_primitives] types for convenience.
pub use alloy_primitives::{address, b256, FixedBytes, B256, U256, U64};

// Testing utils
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

pub mod claims;
#[cfg(feature = "alloc")]
pub mod jwt;
pub mod payload;
pub mod transactions;

mod attributes;
mod blocks;
mod chain;
mod epoch;
mod head;
mod l1_block;
mod peers;
mod str;
mod sync;
mod system;

#[doc(inline)]
pub use attributes::*;
#[doc(inline)]
pub use blocks::*;
#[doc(inline)]
pub use chain::*;
#[doc(inline)]
pub use epoch::*;
#[doc(inline)]
pub use head::*;
#[doc(inline)]
pub use l1_block::*;
#[doc(inline)]
pub use peers::*;
#[doc(inline)]
pub use str::*;
#[doc(inline)]
pub use sync::*;
#[doc(inline)]
pub use system::*;
