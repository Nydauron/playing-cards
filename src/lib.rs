#![warn(missing_docs)]

//! # playing-cards
//!
//! `playing-cards` is a library for developing card games. Ranging from dealing cards from a
//! deck to hand evaluations.
//!
//! ## DISCLAIMER
//! This library is still in early development. A lot of features may be subject to breaking
//! changes across minor version changes. Be sure to check the docs for updates on any changes.
//!
//! ## Breaking Changes from 0.0.4 to 0.1.0
//! - `Rank`s are now primarily used for storing handing rankings instead of `u64`s.
//! - `Rank`s are returned by `Evaluator::evaluate_hand()` instead of a `Vec<u64>`.
//! - `get_string()` has been removed from the `Evaluator` trait. This has been moved to the `Rank`
//! enum.

#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(all(feature = "unstable", test))]
extern crate test;

#[macro_use]
extern crate num_derive;

pub mod core;

#[cfg(feature = "poker")]
pub mod poker;

#[cfg(test)]
mod tests {}
