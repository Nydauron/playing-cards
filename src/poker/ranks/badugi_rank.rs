use std::ops::Deref;

use super::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A rank of a Badugi hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct BadugiRank(pub BasicRank);

impl Deref for BadugiRank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoRankStrengthIterator for BadugiRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(self.0)
    }
}
