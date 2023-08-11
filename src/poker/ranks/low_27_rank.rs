use std::ops::Deref;

use super::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A rank of a 2-to-7 lowball hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Low27Rank(pub BasicRank);

impl Deref for Low27Rank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoRankStrengthIterator for Low27Rank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(self.strength)
    }
}
