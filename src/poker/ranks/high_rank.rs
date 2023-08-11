use std::ops::Deref;

use super::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A rank of a high hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct HighRank(pub BasicRank);

impl Ord for HighRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.strength.cmp(&other.0.strength)
    }
}

impl Deref for HighRank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoRankStrengthIterator for HighRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from((*self).strength)
    }
}
