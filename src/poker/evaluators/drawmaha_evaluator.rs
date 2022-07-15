use async_std::task;

use super::{Evaluator, HighEvaluator, OmahaHighEvaluator, init_lookup_table, EvaluatorError};
use super::super::Rank;

use crate::core::Card;

use rayon::prelude::*;

/// An evaluator for Drawmaha hands.
///
/// Drawmaha is a combination of Five Card Draw and Big O (an Omaha varient). This evaluator makes
/// use of both the HighEvaluator and OmahaHighEvaluator.
pub struct DrawmahaEvaluator;

impl DrawmahaEvaluator {
    /// Creates a new `DrawmahaEvaluator`.
    ///
    /// Initializes the lookup table if it isn't already.
    pub fn new() -> Self {
        task::spawn(async {
            init_lookup_table();
        });
        Self {}
    }
}

impl Evaluator for DrawmahaEvaluator {
    fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Rank>, EvaluatorError> {
        let expected_card_count = 5;
        if player_hand.len() < expected_card_count {
            return Err(EvaluatorError::NotEnoughCards("Player hand".to_string(), 5));
            // Player hand does not have at least 5 cards
        } else if player_hand.len() > expected_card_count {
            return Err(EvaluatorError::TooManyCards("Player hand".to_string(), 5));
            // Player hand does not have at most 5 cards
        }

        if board.len() < 3 {
            return Err(EvaluatorError::NotEnoughCards("Board".to_string(), 3));
            // Board does not have at least 3 cards
        }

        let evals: Vec<Box<dyn Evaluator + Sync>> = vec![Box::new(OmahaHighEvaluator), Box::new(HighEvaluator)];

        evals.par_iter().map(|eval| {
            eval.evaluate_hand(&player_hand, &board)
        })
        .try_reduce(|| Vec::new(), |mut a, b| {
            a.extend(&b);
            Ok(a)
        })
    }
}
