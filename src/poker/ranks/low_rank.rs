//! This module contains the implementation of LowRank.

use super::HighRank;
use std::cmp::Ordering;

/// Distinguhes a hand rank relative to finding the best low hand.
/// 
/// This struct is typically returned by evaluators that evaluate a low hand component.
#[derive(Copy, Clone, Debug)]
pub struct LowRank {
    rank_strength: u16,
    hand_rank: u8,
    sub_rank: u16,
}

impl LowRank {
    /// Creates a new LowRank struct
    pub fn new(strength: u16) -> Self {
        let mut hand_rank: u8 = 0;
        let mut sub_rank: u16 = 0;

        if strength >= 1 {
            let mut ranks_left = strength - 1;

            // distinct combos from high card to straight flush
            let strength_threshold = [1277, 2860, 858, 858, 10, 1277, 156, 156, 10];

            for (i, &subranks) in strength_threshold.iter().enumerate().rev() {
                if ranks_left < subranks {
                    hand_rank = (i + 1) as u8;
                    sub_rank = subranks - ranks_left;
                    break;
                }
                ranks_left -= subranks;
            }
        }

        Self {
            rank_strength: strength,
            hand_rank: hand_rank,
            sub_rank: sub_rank,
        }
    }

    /// Gets the hand rank's strength.
    pub fn get_rank_strength(&self) -> u16 {
        self.rank_strength
    }

    pub fn get_hand_rank(&self) -> u8 {
        self.hand_rank
    }

    pub fn get_sub_rank(&self) -> u16 {
        self.sub_rank
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
        Some(self.get_rank_strength().cmp(&other.get_rank_strength()))
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
