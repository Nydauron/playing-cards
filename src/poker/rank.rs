use std::{cmp::Ordering, hash::Hash, collections::HashSet};

#[derive(Debug, Clone, Eq)]
pub struct Rank {
    pub strength: u32,
    pub hand_rank: u16,
    pub sub_rank: u16,
    pub description: Option<String>,
}

impl Rank {
    pub fn get_strength(&self) -> u32 {
        self.strength
    }

    pub fn get_hand_rank(&self) -> u16 {
        self.hand_rank
    }

    pub fn get_sub_rank(&self) -> u16 {
        self.sub_rank
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

#[derive(Debug, Clone)]
pub struct RankResults<T: Hash + Eq + Copy> {
    hand_table: Vec<Vec<HashSet<T>>>,
}

impl<T: Hash + Eq + Copy> RankResults<T> {
    pub fn new(hand_table: Vec<Vec<HashSet<T>>>) -> Self {
        Self {
            hand_table: hand_table,
        }
    }

    pub fn hand_number(&self, n: usize) -> Option<&Vec<HashSet<T>>> {
        self.hand_table.get(n)
    }
}

impl<T: Hash + Eq + Copy> IntoIterator for RankResults<T> {
    type Item = Vec<HashSet<T>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.hand_table.into_iter()
    }
}
