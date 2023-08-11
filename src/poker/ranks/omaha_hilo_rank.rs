use super::{HighRank, LowA5Rank, IntoRankStrengthIterator, RankStrengthIterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmahaHiLoRank {
    pub hi_rank: HighRank,
    pub lo_rank: Option<LowA5Rank>,
}

impl IntoRankStrengthIterator for OmahaHiLoRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(vec![Some((*self.hi_rank).strength), self.lo_rank.map(|lo| { (*lo).strength })])
    }
}
