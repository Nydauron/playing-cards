//! An optional feature that includes tools for poker hand evaluation.
pub mod tables;
pub use self::tables::*;

pub mod evaluators;
pub use self::evaluators::*;

pub mod ranks;
pub use self::ranks::*;
