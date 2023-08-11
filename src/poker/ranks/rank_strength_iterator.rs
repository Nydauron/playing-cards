use std::collections::HashMap;

use super::BasicRank;

/// Wrapper struct to allow ranks strengths to be iterated across
///
/// Some rank types might have multiple sub ranks of which some might be optional due to the nature
/// of player hand and board combos.
///
/// Iterating through will give `Option<u32>` types, `Some` representing a valid rank and `None`
/// representing a non-existent rank, useful in cases where not every player has a fully qualifying
/// hand (e.g. Omaha Hi-Lo has an low hand criterion but it only applies if the player can make a
/// hand with 5 distinct-rank cards within the range Ace to 8, 2 from theirs and 3 from the board).
pub struct RankStrengthIterator {
    ranks: HashMap<usize, u32>,
    idx: Option<usize>,
    len: usize,
}

impl RankStrengthIterator {
    /// Creates a new iterator
    pub fn new(ranks: HashMap<usize, u32>, num_of_ranks: usize) -> Self {
        Self {
            ranks: ranks,
            idx: None,
            len: num_of_ranks,
        }
    }

    /// Returns the length of the iterator
    pub fn len(&self) -> usize {
        self.len
    }
}

impl From<Vec<Option<u32>>> for RankStrengthIterator {
    fn from(ranks: Vec<Option<u32>>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks
                .into_iter()
                .filter(|x| x.is_some())
                .map(|rank| rank.unwrap())
                .enumerate()
                .collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<Vec<u32>> for RankStrengthIterator {
    fn from(ranks: Vec<u32>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks
                .into_iter()
                .enumerate()
                .collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<u32> for RankStrengthIterator {
    fn from(rank: u32) -> Self {
        Self {
            ranks: HashMap::from([(0, rank)]),
            idx: None,
            len: 1,
        }
    }
}

impl From<Vec<Option<BasicRank>>> for RankStrengthIterator {
    fn from(ranks: Vec<Option<BasicRank>>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks
                .into_iter()
                .filter(|opt_rank| opt_rank.is_some())
                .map(|rank| rank.unwrap().strength)
                .enumerate()
                .collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<Vec<BasicRank>> for RankStrengthIterator {
    fn from(ranks: Vec<BasicRank>) -> Self {
        let len = ranks.len();
        Self {
            ranks: ranks
                .into_iter()
                .map(|rank| rank.strength)
                .enumerate()
                .collect::<HashMap<usize, u32>>(),
            idx: None,
            len,
        }
    }
}

impl From<BasicRank> for RankStrengthIterator {
    fn from(rank: BasicRank) -> Self {
        Self {
            ranks: HashMap::from([(0, rank.strength)]),
            idx: None,
            len: 1,
        }
    }
}

impl Iterator for RankStrengthIterator {
    type Item = Option<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx.map_or(0, |i| i + 1);
        if idx >= self.len {
            self.idx = Some(self.len);
            return None;
        }
        self.idx = Some(idx);

        Some(self.ranks.get(&idx).cloned())
    }
}

/// A trait for converting Rank structs into iterators
///
/// This is a similar implementation as to what IntoIterator does
pub trait IntoRankStrengthIterator {
    /// Creates a `RankStrengthIterator` from a rank type
    fn into_strength_iter(self) -> RankStrengthIterator;
}
