//! An optional feature that includes tools for poker hand evaluation.
mod tables;
pub use self::tables::*;

pub mod evaluators;

/// Contains structs for contains rank metadata and logic for parsing ranks.
pub mod rank;
