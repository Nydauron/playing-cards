use super::{HighRank, IntoRankStrengthIterator, RankStrengthIterator};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct DrawmahaRank{
    pub omaha_rank: HighRank,
    pub draw_rank: HighRank,
}

impl IntoRankStrengthIterator for DrawmahaRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(vec![(*self.omaha_rank).strength, (*self.draw_rank).strength])
    }
}
