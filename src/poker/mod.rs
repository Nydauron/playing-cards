//! An optional feature that includes tools for poker hand evaluation.
pub(crate) mod tables;

pub mod evaluators;

/// Contains structs for contains rank metadata and logic for parsing ranks.
pub mod rank;

pub mod evaluator_result;
