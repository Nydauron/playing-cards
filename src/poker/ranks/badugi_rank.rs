use super::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct BadugiRank(pub BasicRank);

impl Ord for BadugiRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.strength.cmp(&other.0.strength)
    }
}

impl IntoRankStrengthIterator for BadugiRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(self.0)
    }
}
