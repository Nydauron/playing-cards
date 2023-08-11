use std::error;
#[allow(unused_imports)]
use std::error::Error as _;
use std::fmt;

/// An error wrapper that provides error handling for the evaluators.
#[derive(Debug)]
pub enum EvaluatorError {
    /// This error represents when there are not enough cards provided to the evaluator
    ///
    /// Params:
    /// card_set_type: String - what set of cards had not enough cards
    /// expected_count: u64 - the expected amount of cards (at least)
    NotEnoughCards(String, u64),
    /// This error represents when there are too many cards provided to the evaluator
    ///
    /// Params:
    /// card_set_type: String - what set of cards had not enough cards
    /// expected_count: u64 - the expected amount of cards (at most)
    TooManyCards(String, u64),
    /// An unknown error that should not normally occur
    ///
    /// This is typically a bandaid solution for current evaluators and contains less-concrete
    /// errors.
    UnknownError(String),
}

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EvaluatorError::*;

        match self {
            NotEnoughCards(set_type, expected_at_least) => {
                let at_least_plural = if *expected_at_least == 1 { "" } else { "s" };
                write!(
                    f,
                    "{} does not have at least {} card{}",
                    set_type, expected_at_least, at_least_plural
                )
            }
            TooManyCards(set_type, expected_no_more) => {
                let at_least_plural = if *expected_no_more == 1 { "" } else { "s" };
                write!(
                    f,
                    "{} does not have at most {} card{}",
                    set_type, expected_no_more, at_least_plural
                )
            }
            UnknownError(msg) => {
                write!(f, "Uh oh! An unknown error occurred!\n{}", msg)
            }
        }
    }
}

impl error::Error for EvaluatorError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use EvaluatorError::*;

        match *self {
            NotEnoughCards(_, _) => None,
            TooManyCards(_, _) => None,
            UnknownError(_) => None,
        }
    }
}
