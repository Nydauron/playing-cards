#![warn(missing_docs)]

//! # playing-cards
//! 
//! `playing-cards` is a library for developing card games. Ranging from dealing cards from a
//! deck to hand evaluations.

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
