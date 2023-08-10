use std::ops::Deref;

use crate::poker::{rank::BasicRank, evaluator_result::{IntoRankStrengthIterator, RankStrengthIterator}};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct LowA5Rank(pub BasicRank);

impl Ord for LowA5Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.strength.cmp(&other.0.strength)
    }
}

impl Deref for LowA5Rank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }    
}
impl IntoRankStrengthIterator for LowA5Rank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from((*self).clone())
    }
}

