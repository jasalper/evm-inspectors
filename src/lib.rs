//! revm [Inspector](revm::Inspector) implementations, such as call tracers
//!
//! ## Feature Flags
//!
//! - `js-tracer`: Enables a JavaScript tracer implementation. This pulls in extra dependencies
//!   (such as `boa`, `tokio` and `serde_json`).

#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// An inspector implementation for an EIP2930 Accesslist
pub mod access_list;

/// An inspector stack abstracting the implementation details of
/// each inspector and allowing to hook on block/transaction execution,
/// used in the main RETH executor.
pub mod stack;

/// An inspector for recording traces
pub mod tracing;
