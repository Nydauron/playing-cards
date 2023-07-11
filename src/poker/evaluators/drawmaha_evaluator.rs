use async_std::task;

use super::{Evaluator, HighEvaluator, OmahaHighEvaluator, EvaluatorError};
use super::super::Rank;

use crate::core::Card;

use rayon::prelude::*;

/// An evaluator for Drawmaha hands.
///
/// Drawmaha is a combination of Five Card Draw and Big O (an Omaha varient). This evaluator makes
/// use of both the HighEvaluator and OmahaHighEvaluator.
///
/// Examples
/// ```rust
/// use playing_cards::{core::Card, poker::{Evaluator, DrawmahaEvaluator, Rank}};
///
/// let hand = Card::vec_from_str("5cAsKdKcAc").unwrap();
/// let board = Card::vec_from_str("Ks6s2d8c3h").unwrap();
///
/// let eval = DrawmahaEvaluator::new();
///
/// let rank = eval.evaluate_hand(&hand, &board).unwrap();
/// let omaha_rank = rank[0];
/// let draw_rank = rank[1];
///
/// assert_eq!(omaha_rank.get_string().unwrap(), "Trip Kings");
/// assert_eq!(draw_rank.get_string().unwrap(), "Two Pair of Aces and Kings");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::{Evaluator, DrawmahaEvaluator, Rank}};
///
/// let hand = Card::vec_from_str("3s9sAsTsQs").unwrap();
/// let board = Card::vec_from_str("4d9hQdTcKh").unwrap();
///
/// let eval = DrawmahaEvaluator::new();
///
/// let rank = eval.evaluate_hand(&hand, &board).unwrap();
/// let omaha_rank = rank[0];
/// let draw_rank = rank[1];
///
/// assert_eq!(omaha_rank.get_string().unwrap(), "Two Pair of Queens and 10s");
/// assert_eq!(draw_rank.get_string().unwrap(), "Ace High Flush");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::{Evaluator, DrawmahaEvaluator, Rank}};
///
/// let hero_hand = Card::vec_from_str("Tc9sJs8hQd").unwrap();
/// let villan_hand = Card::vec_from_str("AsQcKdQhAc").unwrap();
/// let board = Card::vec_from_str("8d8s3cAh7d").unwrap();
///
/// let eval = DrawmahaEvaluator::new();
///
/// let hero_rank = eval.evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = eval.evaluate_hand(&villan_hand, &board).unwrap();
///
/// // Omaha Rank
/// assert_eq!(hero_rank[0].get_string().unwrap(), "Trip 8s");
/// assert_eq!(villan_rank[0].get_string().unwrap(), "Aces Full of 8s");
///
/// assert!(hero_rank[0] < villan_rank[0]); // Villan's hand is better than the hero's
///
/// // 5-card Draw Rank
/// assert_eq!(hero_rank[1].get_string().unwrap(), "Queen High Straight");
/// assert_eq!(villan_rank[1].get_string().unwrap(), "Two Pair of Aces and Queens");
///
/// assert!(hero_rank[1] > villan_rank[1]); // Hero's hand is better than the villan's
/// ```
pub struct DrawmahaEvaluator;

impl DrawmahaEvaluator {
    /// Creates a new `DrawmahaEvaluator`.
    ///
    /// Initializes the lookup table if it isn't already.
    pub fn new() -> Self {
        Self {}
    }
}

impl Evaluator for DrawmahaEvaluator {
    /// Evaluated the Drawmaha for one player.
    ///
    /// Returns a `Vec<OmahaHighRank, HighRank>` that can be compared directly against other
    /// `OmahaHighRank`s and `HighRanks`s respectfully. If the player's hand does not contain
    /// exactly 5 cards or the board contains less than 3 cards, then an error will return.
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

        let evals: Vec<(Box<dyn Evaluator + Sync>, Vec<Card>, Vec<Card>)> = vec![
            (Box::new(OmahaHighEvaluator), player_hand.clone(), board.clone()),
            (Box::new(HighEvaluator), player_hand.clone(), Vec::new()),
        ];

        evals.par_iter().map(|(eval, player_hand, board)| {
            eval.evaluate_hand(&player_hand, &board)
        })
        .try_reduce(|| Vec::new(), |mut a, b| {
            a.extend(&b);
            Ok(a)
        })
    }
}
