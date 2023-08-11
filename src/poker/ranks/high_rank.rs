use std::ops::Deref;

use super::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A rank of a high hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct HighRank(pub BasicRank);

impl Deref for HighRank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoRankStrengthIterator for HighRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(self.strength)
    }
}
