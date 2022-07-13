//! This module contains the implementation of LowRank.

use super::HighRank;
use std::cmp::Ordering;

/// Distinguhes a hand rank relative to finding the best low hand.
/// 
/// This struct is typically returned by evaluators that evaluate a low hand component.
#[derive(Copy, Clone, Debug)]
pub struct LowRank {
    rank_strength: u64,
}

impl LowRank {
    /// Creates a new LowRank struct
    pub fn new(strength: u64) -> Self {
        Self { rank_strength: strength }
    }

    /// Gets the hand rank's strength.
    pub fn get_rank_strength(&self) -> u64 {
        self.rank_strength
    }

    /// Returns the string for the associated hand.
    ///
    /// The string is user-interperable string of the hand strength and can be used for displaying
    /// to the user.
    pub fn get_string(&self) -> Result<String, &'static str> {
        let high_rank = HighRank::new(self.rank_strength);
        high_rank.get_string()
    }
}

// PartialOrd and PartialEq unfortunately are repeated for all Rank types
// This is because there is no way to implement generic types for foriegn traits, so alas
impl PartialOrd for LowRank {
    fn partial_cmp(&self, other: &LowRank) -> Option<Ordering> {
        // We can do a little trick here:
        // If x is a higher hand than y, then partial_cmp(x, y) should return Greater.
        // Therefore, we can determine that partial_cmp(x, y) and partial_cmp(y, x) should return
        // Greater and Less respectively. We can then say that if partial_cmp() returns Greater,
        // the LHS is better and if it returns Less, the RHS is better.
        //
        // If we flip x and y, then we determine what's the low hand is evaluated since it flips
        // the range. If x is a higher hand than y, then partial_cmp(x, y) returns Less, implying
        // the better hand is y.

        Some(other.get_rank_strength().cmp(&self.get_rank_strength()))
    }
}

impl PartialEq for LowRank {
    fn eq(&self, other: &LowRank) -> bool {
        self.get_rank_strength() == other.get_rank_strength()
    }

    fn ne(&self, other: &LowRank) -> bool {
        self.get_rank_strength() != other.get_rank_strength()
    }
}
