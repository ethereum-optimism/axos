//! # Providers
//!
//! This crate contains a set of providers for [axos]. Providers are
//! built in a `no_std` environment.
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

#[cfg(feature = "test-utils")]
pub mod mock;
pub mod provider;
pub mod purple;
