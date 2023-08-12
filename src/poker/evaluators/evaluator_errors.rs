use num_traits::{One, PrimInt};

fn pluralize<T: PrimInt + One>(n: T, base: &str, plural_suffix: &str) -> String {
    if n.is_one() {
        base.to_string()
    } else {
        base.to_string() + plural_suffix
    }
}

/// An error wrapper that provides error handling for the evaluators
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum EvaluatorError {
    /// There are not enough cards provided to the evaluator
    #[non_exhaustive]
    #[error("{card_set_type} does not have at least {expected_count} {} (Got instead {actual_count} {})", pluralize(*.expected_count, "card", "s"), pluralize(*.actual_count, "card", "s"))]
    NotEnoughCards {
        /// What set of cards had not enough cards
        card_set_type: String,
        /// The expected amount of cards (at least)
        expected_count: u64,
        /// The actual amount of cards recieved
        actual_count: u64,
    },
    /// There are too many cards provided to the evaluator
    #[non_exhaustive]
    #[error("{card_set_type} does not have at most {expected_count} {} (Got instead {actual_count} {})", pluralize(*.expected_count, "card", "s"), pluralize(*.actual_count, "card", "s"))]
    TooManyCards {
        /// What set of cards had not enough cards
        card_set_type: String,
        /// The expected amount of cards (at most)
        expected_count: u64,
        /// The actual amount of cards recieved
        actual_count: u64,
    },
    /// The evalautor that was called was unable to find a valid rank for the given hand
    #[non_exhaustive]
    #[error("Failed to calculate rank based off of set of cards: {0}")]
    FailedToCalculateRank(String),
}
