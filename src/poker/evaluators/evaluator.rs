use crate::{core::Card, poker::Rank};


use super::EvaluatorError;



/// A Trait definition for all poker evaluators.
pub trait Evaluator {

    /// Evaluates a hand for one player.
    fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Rank>, EvaluatorError>;
}
