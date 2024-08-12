#![warn(missing_docs)]

//! # playing-cards
//!
//! `playing-cards` is a library for developing card games, ranging from dealing cards from a
//! deck to hand evaluations.
//!
//! ## DISCLAIMER
//! This library is still in early development. A lot of features may be subject to breaking
//! changes across minor version changes. Be sure to check the docs for updates on any changes.
//!
//! ## MSRV (Minimum Supported Rust Version)
//! Requires rustc 1.62+
//!
//! ## Breaking Changes in v0.1.0
//! - Rank types are now primarily used for storing hand rankings instead of `u64`s.
//! - The `Evaluator` trait has been removed.
//! - All evaluators in `poker::evaluators` have changed to be standalone functions rather than
//!   structs. Evaluator functions can differ in signature, primarily due return type, but also
//!   arguments.
//! - `get_string()` has been removed in favor of having the rank string computed on `Rank`
//!   construction. Please see `BasicRank::description`.
//!
//! ## Some extra changes in v0.1.0
//! - The High Evaluator now uses the Cactus-Key Perfect Hash algorithm to calculate rank strength.
//!   While the 2+2 evaluator is in theory faster, compilation times for this library were extremely
//!   high when generating the graph, the amount of RAM and disk space of containing such codegen was
//!   too impractical and did not outweigh the minor performance improvement.
//! - `poker::ranks::generate_winner_list()` can be used to calculate the winners given the
//!   associated ranks of each player.
//! - `core::CardDeck` now uses the Xoshiro256PlusPlus PRNG instead of SFMT and now requires a
//!   256-bit seed rather than a 64-bit seed.
//!   - The reason why the seed width has increased is to ensure a chance for all possible deck
//!     permutations. A 64-bit key only has the ability of replicating a very small fraction of these
//!     permutations. In terms of likelihood, this shouldn't drastically change your likelihood of
//!     getting a straight flush, but it brings it a bit closer to how a normal deck would function
//!     in real life.
//! - Added the following evaluators:
//!   - 2-7 Low
//!   - Omaha Hi
//!   - Omaha Hi-Lo
//!   - Dramaha High
//!   - Badugi
//!
//! ## Features
//! * `core`: A default feature that includes the core module
//! * `poker`: The poker module
//! * `serde`: Adds serde-related traits to structs

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
