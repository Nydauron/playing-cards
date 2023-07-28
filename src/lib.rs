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
//! ## Breaking Changes from v0.0.4 to v0.1.0
//! - `Rank`s are now primarily used for storing handing rankings instead of `u64`s.
//! - All evaluators in `poker::evaluators` have changed to be standalone functions rather than
//! structs.
//! - The `Evaluator` trait has been removed.
//! - `Vec<Rank>`s are returned by all `evaluate_hand()` functions instead of a `Vec<u64>`.
//! - `get_string()` has been removed in favor of having the rank string computed on `Rank`
//! construction.
//!
//! ## Some extra changes in v0.1.0
//! - The High Evaluator now uses the Cactus-Key Perfect Hash algorithm to calculate rank strength.
//! While the 2+2 evaluator is in theory faster, compilation times for this library were extremely
//! high when generating the graph, the amount of RAM and disk space of containing such codegen was
//! too impratical and did not outweigh the minor performance improvement.
//! - `poker::rank::generate_winner_list()` can be used to calculate the winners given the
//! associated ranks of each player.
//! - `core::CardDeck` now uses the Xoshiro256PlusPlus PRNG instead of SFMT and now requires a
//! 256-bit seed rather than a 64-bit seed.
//!   - The reason why the seed width has increased is to ensure a chance for all possible deck
//!   permutations. A 64-bit key only has the ability of replicating a very small fraction of these
//!   permutations. In terms of likelihood, this shouldn't drastically change your likelihood of
//!   getting a straight flush, but it brings it a bit closer to how a normal deck would function
//!   in real life.

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
