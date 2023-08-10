use crate::{core::Card, poker::rank::Rank};

use super::{high_evaluator, EvaluatorError, low_27_evaluator, omaha_hi_evaluator, omaha_hilo_evaluator, drawmaha_evaluator};

/// Contains all evaluator variants allowing the ability to switch between evaluators.
#[derive(Copy, Clone, Debug)]
pub enum Evaluator {
    /// Enum variant representing high_evaluator
    High,
    /// Enum variant representing low_27_evaluator
    Low27,
    /// Enum variant representing omaha_hi_evaluator
    OmahaHi,
    /// Enum variant representing omaha_hilo_evaluator
    OmahaHiLo,
    /// Enum variant representing drawmaha_evaluator
    Drawmaha,
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
            Self::Low => Ok(vec![Some(low_27_evaluator::evaluate_hand(player_hand, board)?)]),
            Self::OmahaHi => Ok(vec![Some(omaha_hi_evaluator::evaluate_hand(player_hand, board)?)]),
            Self::OmahaHiLo => omaha_hilo_evaluator::evaluate_hand(player_hand, board),
            Self::Drawmaha => Ok(drawmaha_evaluator::evaluate_hand(player_hand, board)?.iter().cloned().map(|rank| Some(rank)).collect::<Vec<_>>()),
        }
    }
}
