use std::collections::HashMap;

use super::BasicRank;

pub struct RankStrengthIterator {
    ranks: HashMap<usize, u32>,
    idx: Option<usize>,
    len: usize,
}

impl RankStrengthIterator {
    pub fn new(ranks: HashMap<usize, u32>, num_of_ranks: usize) -> Self {
        Self {
            ranks: ranks,
            idx: None,
            len: num_of_ranks,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl From<Vec<Option<u32>>> for RankStrengthIterator {
    fn from(ranks: Vec<Option<u32>>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks.into_iter().filter(|x| x.is_some()).map(|rank| rank.unwrap()).enumerate().collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<Vec<u32>> for RankStrengthIterator {
    fn from(ranks: Vec<u32>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks.into_iter().enumerate().collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<u32> for RankStrengthIterator {
    fn from(rank: u32) -> Self {
        Self { ranks: HashMap::from([(0, rank)]), idx: None, len: 1 }
    }
}

impl From<Vec<Option<BasicRank>>> for RankStrengthIterator {
    fn from(ranks: Vec<Option<BasicRank>>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks.into_iter().filter(|opt_rank| opt_rank.is_some()).map(|rank| rank.unwrap().strength).enumerate().collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<Vec<BasicRank>> for RankStrengthIterator {
    fn from(ranks: Vec<BasicRank>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks.into_iter().map(|rank| rank.strength).enumerate().collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<BasicRank> for RankStrengthIterator {
    fn from(rank: BasicRank) -> Self {
        Self { ranks: HashMap::from([(0, rank.strength)]), idx: None, len: 1 }
    }
}

impl Iterator for RankStrengthIterator {
    type Item = Option<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx.map_or(0, |i| i+1);
        if idx >= self.len {
            self.idx = Some(self.len);
            return None;
        }
        self.idx = Some(idx);

        Some(self.ranks.get(&idx).cloned())
    }
}

pub trait IntoRankStrengthIterator {
    fn into_strength_iter(self) -> RankStrengthIterator;
}
