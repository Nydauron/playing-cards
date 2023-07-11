//! This module contains all Rank types and traits.

use std::fmt::Debug;
use std::cmp::Ordering;
use super::{HighRank, LowRank};

/// The `Rank` enum encompasses all rank types.
///
/// Current implementations are for:
/// - High ranks
/// - Low ranks
///
/// A `Rank` varient can be compared with any other of the same varient.
#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank {
    High(HighRank),
    Low(LowRank),
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Rank::*;

        match (self, other) {
            (&High(h1), &High(h2)) => h1.partial_cmp(&h2),
            (&Low(l1), &Low(l2)) => l1.partial_cmp(&l2),
            _ => None,
        }
    }
}

impl Rank {
    /// Gets the hand rank's strength.
    pub fn get_rank_strength(&self) -> u64 {
        use Rank::*;

        match self {
            &High(h) => h.get_rank_strength(),
            &Low(l) => l.get_rank_strength(),
        }
    }

    /// Returns the string for the associated hand.
    ///
    /// The string is user-interperable string of the hand strength and can be used for displaying
    /// to the user.
    pub fn get_string(&self) -> Result<String, &'static str> {
        use Rank::*;

        match self {
            &High(h) => h.get_string(),
            &Low(l) => l.get_string(),
        }
    }
}
