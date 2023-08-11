use super::{HighRank, IntoRankStrengthIterator, RankStrengthIterator};

/// A struct of ranks a Drawmaha hand
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct DrawmahaRank{
    /// The Omaha high rank from the hand
    pub omaha_rank: HighRank,
    /// The five-card draw rank from the hand
    pub draw_rank: HighRank,
}

impl IntoRankStrengthIterator for DrawmahaRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(vec![(*self.omaha_rank).strength, (*self.draw_rank).strength])
    }
}
