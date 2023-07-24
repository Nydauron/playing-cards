use super::{high_evaluator, EvaluatorError, omaha_hi_evaluator};
use super::super::rank::Rank;

use crate::core::Card;

/// Evaluated the Drawmaha for one player.
///
/// Returns a `Vec<Rank>` where the first element is the rank for the Omaha hand and the second
/// element is for the draw hand. If the player's hand does not contain exactly 5 cards or the
/// board contains less than 3 cards, then an error will return.
pub fn evaluate_hand(player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Rank>, EvaluatorError> {
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

    let omaha_hand_rank = match omaha_hi_evaluator::evaluate_hand(player_hand, board) {
        Ok(rank) => rank,
        Err(e) => return Err(e),
    };
    let draw_hand_rank = match high_evaluator::evaluate_hand(player_hand, &vec![]) {
        Ok(rank) => rank,
        Err(e) => return Err(e),
    };

    Ok(omaha_hand_rank.into_iter().chain(draw_hand_rank.into_iter()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_house_omaha_two_pair_draw() {
        let player_hand = Card::vec_from_str("AsKcAdQhQc").unwrap();
        let board = Card::vec_from_str("KhQsJdKdJs").unwrap();

        let player_ranks = evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_ranks: Vec<String> = player_ranks.iter().map(|rank| rank.description.as_ref().expect("Hand generated bad rank").to_owned() ).collect();
        assert_eq!(vec!["Kings Full of Queens".to_string(), "Two Pair of Aces and Queens".to_string()], string_ranks);
    }

    #[test]
    fn two_pair_omaha_high_card_draw() {
        let player_hand = Card::vec_from_str("AsQh2h5d7d").unwrap();
        let board = Card::vec_from_str("3s8sKs3dQs").unwrap();

        let player_ranks = evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_ranks: Vec<String> = player_ranks.into_iter().map(|rank| rank.description.as_ref().expect("Hand generated bad rank").to_owned() ).collect();
        assert_eq!(vec!["Two Pair of Queens and 3s".to_string(), "Ace High".to_string()], string_ranks);
    }
}
