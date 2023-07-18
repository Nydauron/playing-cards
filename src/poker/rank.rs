use std::{cmp::Ordering, hash::Hash, collections::HashSet};

/// A struct to contain hand strength metadata.
///
/// The `Rank` struct is fairly trasparent allowing for easy access for evaluators to create and
/// modify values within the struct. The struct primarily stores information on hand strength, even
/// containing information on hand rank and sub rank if the user wants to utilize those properites.
///
/// A provided description is also included to allow for the user to understand what the rank is in
/// English. Typically, the `description` field is going to be the the strength of the hand, but if
/// the evaluator fails for whatever reason, it will contain an error message instead.
///
/// Ranks can be compared with each other using the typical equality and inequality operations. Do
/// note that two ranks from two different evaluators can be compared successfully which might lead
/// to some undefined behavior in the user's implementation.
#[derive(Debug, Clone, Eq)]
pub struct Rank {
    /// The strength of the `Rank`.
    ///
    /// This field is what is used within the implematation of `Ord`
    /// and `Eq`.
    pub strength: u32,

    /// the hand rank of the associated rank.
    ///
    /// Two different ranks from two different evaluators might have the same `strength`, they may
    /// differ in `hand_rank`.
    pub hand_rank: u16,

    /// The sub rank of the associated rank.
    ///
    /// Typically `sub_rank` is used in tandem with `hand_rank`.
    pub sub_rank: u16,

    /// An optional description for the rank.
    ///
    /// All the provided evaluators in this library fill this field. Typically, value that is
    /// populated is a Enlgish version of the hand made (e.g. "Trip 5s", "Kings full of 9s"). If
    /// the evaluator fails to generate a valid description, the error string will be populated
    /// here.
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

/// Contains information on which key has the strongest rank (i.e the winning hand).
///
/// The struct does not contain any `Rank` types, but rather contains the keys associated with the
/// rank. The underlying implementation uses HashSets to store the keys so the generic type `T`
/// must fulfill the traits `Hash`, `Eq`, and `Copy`.
///
/// The vectors returned by the iterator or `hand_number(&self)` is a list of hashsets of keys. The
/// list is sorted by strength of the key's rank with index 0 being of the strongest rank.
#[derive(Debug, Clone)]
pub struct RankResults<T: Hash + Eq + Copy> {
    hand_table: Vec<Vec<HashSet<T>>>,
}

impl<T: Hash + Eq + Copy> RankResults<T> {
    /// Generates a new `RankResults<T>`.
    ///
    /// This does not do any other post-computation. This is typically called within the
    /// `generate_winner_list()` functions.
    pub fn new(hand_table: Vec<Vec<HashSet<T>>>) -> Self {
        Self {
            hand_table: hand_table,
        }
    }

    /// Gets the sepcified hand number if it exists.
    ///
    /// Similar to indexing an vector, this will either return a list of hashsets of keys which are
    /// sorted by the strength of the key's rank, or None if the index does not exist.
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
