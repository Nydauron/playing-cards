use super::{HighRank, LowA5Rank, IntoRankStrengthIterator, RankStrengthIterator};

/// A struct of for a given Omaha Hi-Lo hand
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmahaHiLoRank {
    /// The Omaha hi rank from the hand
    pub hi_rank: HighRank,
    /// The Omaha lo rank from the hand
    ///
    /// Note that not all Omaha hands + boards will result in a lo hand.
    pub lo_rank: Option<LowA5Rank>,
}

impl IntoRankStrengthIterator for OmahaHiLoRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(vec![Some((*self.hi_rank).strength), self.lo_rank.map(|lo| { (*lo).strength })])
    }
}
