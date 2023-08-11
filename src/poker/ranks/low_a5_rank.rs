use std::ops::Deref;

use crate::poker::ranks::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A rank of a Ace-to-5 lowball hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct LowA5Rank(pub BasicRank);

impl Deref for LowA5Rank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl IntoRankStrengthIterator for LowA5Rank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(self.strength)
    }
}
