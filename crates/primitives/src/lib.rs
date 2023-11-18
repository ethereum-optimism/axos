#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Re-export used [alloy_primitives] types for convenience.
pub use alloy_primitives::{address, b256, B256, U256};

// Testing utils
#[cfg(feature = "test-utils")]
pub mod test_utils;

mod blocks;
mod chain;
mod info;
mod peers;
mod sync;
mod system;

#[doc(inline)]
pub use blocks::*;
#[doc(inline)]
pub use chain::*;
#[doc(inline)]
pub use info::*;
#[doc(inline)]
pub use peers::*;
#[doc(inline)]
pub use sync::*;
#[doc(inline)]
pub use system::*;
