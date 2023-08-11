use std::cmp::Ordering;

/// A fundational struct to contain hand strength metadata
///
/// The `BasicRank` struct is fairly trasparent allowing for easy access for evaluators to create
/// and modify values within the struct. The struct primarily stores information on hand strength,
/// even containing information on hand rank and sub rank if the user wants to utilize those
/// properites.
///
/// A provided description is also included to allow for the user to understand what the rank is in
/// English. Typically, the `description` field is going to be the the strength of the hand, but if
/// the evaluator fails for whatever reason, it will contain an error message instead.
///
/// Ranks can be compared with each other using the typical equality and inequality operations.
/// The evaluators that are provided in the `evaluator` module produce structs that rely on this
/// foundational struct.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasicRank {
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

impl Ord for BasicRank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for BasicRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
