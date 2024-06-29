use std::cmp::Ordering;

use super::{high_evaluator, omaha_hi_evaluator, EvaluatorError};

use crate::core::Card;
use crate::poker::ranks::DramahaHighRank;

/// Evaluates the Dramaha high hand for one player
///
/// Returns a `DramahaHighRank` where the first element is the rank for the Omaha hand and the
/// second element is for the draw hand. If the player's hand does not contain exactly 5 cards or
/// the board contains less than 3 cards, then an error will return.
///
/// This implementation does not support the use of duplicate cards. If duplicate cards are found
/// when both the player's cards and the board are chained, a `FailedToCalculateRank` error will
/// return.
pub fn evaluate_hand(
    player_hand: &Vec<Card>,
    board: &Vec<Card>,
) -> Result<DramahaHighRank, EvaluatorError> {
    let expected_card_count = 5;
    match player_hand.len().cmp(&expected_card_count) {
        Ordering::Less => Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Player hand".to_string(),
            expected_count: expected_card_count as u64,
            actual_count: player_hand.len() as u64,
        }),
        Ordering::Greater => Err(EvaluatorError::TooManyCards {
            card_set_type: "Player hand".to_string(),
            expected_count: expected_card_count as u64,
            actual_count: player_hand.len() as u64,
        }),
        Ordering::Equal => {
            if board.len() < 3 {
                return Err(EvaluatorError::NotEnoughCards {
                    card_set_type: "Board".to_string(),
                    expected_count: 3,
                    actual_count: board.len() as u64,
                });
            }

            let omaha_hand_rank = omaha_hi_evaluator::evaluate_hand(player_hand, board)?;
            let draw_hand_rank = high_evaluator::evaluate_hand(player_hand)?;

            Ok(DramahaHighRank {
                omaha_rank: omaha_hand_rank,
                draw_rank: draw_hand_rank,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_house_omaha_two_pair_draw() {
        let player_hand = Card::vec_from_str("AsKcAdQhQc").unwrap();
        let board = Card::vec_from_str("KhQsJdKdJs").unwrap();

        let player_ranks = evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_ranks = vec![
            player_ranks.omaha_rank.description.as_ref().unwrap(),
            player_ranks.draw_rank.description.as_ref().unwrap(),
        ];
        assert_eq!(
            vec!["Kings Full of Queens", "Two Pair of Aces and Queens"],
            string_ranks
        );
    }

    #[test]
    fn two_pair_omaha_high_card_draw() {
        let player_hand = Card::vec_from_str("AsQh2h5d7d").unwrap();
        let board = Card::vec_from_str("3s8sKs3dQs").unwrap();

        let player_ranks = evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_ranks = vec![
            player_ranks.omaha_rank.description.as_ref().unwrap(),
            player_ranks.draw_rank.description.as_ref().unwrap(),
        ];
        assert_eq!(vec!["Two Pair of Queens and 3s", "Ace High"], string_ranks);
    }
}
