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

/// Re-export [axos_primitives] for convenience.
pub use axos_primitives::*;

pub mod config;
pub mod info;
pub mod ingest;
