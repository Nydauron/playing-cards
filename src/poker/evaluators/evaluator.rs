use crate::{core::Card, poker::rank::Rank};

use super::{high_evaluator, EvaluatorError, low_evaluator};

/// Contains all evaluator variants allowing the ability to switch between evaluators.
#[derive(Copy, Clone, Debug)]
pub enum Evaluator {
    /// Enum variant representing high_evaluator
    High,
    /// Enum variant representing low_evaluator
    Low,
}

impl Evaluator {
    /// Calls the associated `evaluate_hand()` function based on what variant is currently
    /// selected.
    ///
    /// This function acts as a wrapper function to allow for interfacing all evaluators. Some
    /// evaluators will always return back a rank if no error occurs, others might not return back
    /// a rank if the rank is secondary.
    pub fn evaluate_hand(self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Option<Rank>>, EvaluatorError> {
        match self {
            Self::High => Ok(vec![Some(high_evaluator::evaluate_hand(player_hand, board)?)]),
            Self::Low => Ok(vec![Some(low_evaluator::evaluate_hand(player_hand, board)?)]),
        }
    }
}
