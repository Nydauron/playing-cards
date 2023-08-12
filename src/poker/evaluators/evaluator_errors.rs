use num_traits::{One, PrimInt};

fn pluralize<T: PrimInt + One>(n: T, base: &str, plural_suffix: &str) -> String {
    if n.is_one() {
        base.to_string()
    } else {
        base.to_string() + plural_suffix
    }
}

/// An error wrapper that provides error handling for the evaluators
#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    /// This error represents when there are not enough cards provided to the evaluator
    #[error("{card_set_type} does not have at least {expected_count} {}", pluralize(*.expected_count, "card", "s"))]
    NotEnoughCards {
        /// What set of cards had not enough cards
        card_set_type: String,
        /// The expected amount of cards (at least)
        expected_count: u64,
    },
    /// This error represents when there are too many cards provided to the evaluator
    #[error("{card_set_type} does not have at most {expected_count} {}", pluralize(*.expected_count, "card", "s"))]
    TooManyCards {
        /// What set of cards had not enough cards
        card_set_type: String,
        /// The expected amount of cards (at most)
        expected_count: u64,
    },
    /// An unknown error that should not normally occur
    ///
    /// This is typically a bandaid solution for current evaluators and contains less-concrete
    /// errors.
    #[error("Uh oh! An unknown error occurred!\n{0}")]
    UnknownError(String),
}
