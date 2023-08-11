use super::high_evaluator::HighRank;
use super::{high_evaluator, EvaluatorError, omaha_hi_evaluator};

use crate::core::Card;
use crate::poker::evaluator_result::{IntoRankStrengthIterator, RankStrengthIterator};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct DrawmahaRank{
    pub omaha_rank: HighRank,
    pub draw_rank: HighRank,
}

impl IntoRankStrengthIterator for DrawmahaRank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from(vec![(*self.omaha_rank).strength, (*self.draw_rank).strength])
    }
}

/// Evaluated the Drawmaha for one player.
///
/// Returns a `Vec<Rank>` where the first element is the rank for the Omaha hand and the second
/// element is for the draw hand. If the player's hand does not contain exactly 5 cards or the
/// board contains less than 3 cards, then an error will return.
pub fn evaluate_hand(player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<DrawmahaRank, EvaluatorError> {
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

    let omaha_hand_rank = omaha_hi_evaluator::evaluate_hand(player_hand, board)?;
    let draw_hand_rank = high_evaluator::evaluate_hand(player_hand)?;

    Ok(DrawmahaRank { omaha_rank: omaha_hand_rank, draw_rank: draw_hand_rank })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_house_omaha_two_pair_draw() {
        let player_hand = Card::vec_from_str("AsKcAdQhQc").unwrap();
        let board = Card::vec_from_str("KhQsJdKdJs").unwrap();

        let player_ranks = evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_ranks = vec![(*player_ranks.omaha_rank).description.as_ref().unwrap(), (*player_ranks.draw_rank).description.as_ref().unwrap()];
        assert_eq!(vec!["Kings Full of Queens", "Two Pair of Aces and Queens"], string_ranks);
    }

    #[test]
    fn two_pair_omaha_high_card_draw() {
        let player_hand = Card::vec_from_str("AsQh2h5d7d").unwrap();
        let board = Card::vec_from_str("3s8sKs3dQs").unwrap();

        let player_ranks = evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_ranks = vec![(*player_ranks.omaha_rank).description.as_ref().unwrap(), (*player_ranks.draw_rank).description.as_ref().unwrap()];
        assert_eq!(vec!["Two Pair of Queens and 3s", "Ace High"], string_ranks);
    }
}
