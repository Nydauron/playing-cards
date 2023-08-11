use std::ops::Deref;

use super::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A rank of a 2-to-7 lowball hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Low27Rank(pub BasicRank);

impl Ord for Low27Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.strength.cmp(&other.0.strength)
    }
}

impl Deref for Low27Rank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoRankStrengthIterator for Low27Rank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from((*self).strength)
    }
}
