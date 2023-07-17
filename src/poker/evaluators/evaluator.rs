use crate::{core::Card, poker::rank::Rank};

use super::{high_evaluator, EvaluatorError, low_evaluator};

#[derive(Copy, Clone, Debug)]
pub enum Evaluator {
    High,
    Low,
}

impl Evaluator {
    pub fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Rank>, EvaluatorError> {
        match self {
            Self::High => high_evaluator::evaluate_hand(player_hand, board),
            Self::Low => low_evaluator::evaluate_hand(player_hand, board),
        }
    }
}
