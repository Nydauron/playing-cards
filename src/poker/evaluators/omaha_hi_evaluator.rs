use super::EvaluatorError;

use itertools::Itertools;

use crate::core::Card;
use crate::poker::evaluators::high_evaluator;
use crate::poker::ranks::HighRank;

/// Evaluates the Omaha high hand for one player.
///
/// Returns a `HighRank`. If the player's hand contains less than 4 cards or the board contains
/// less than 3 cards, then an error will return.
pub fn evaluate_hand(
    player_hand: &Vec<Card>,
    board: &Vec<Card>,
) -> Result<HighRank, EvaluatorError> {
    if player_hand.len() < 4 {
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Player hand".to_string(),
            expected_count: 4,
        });
        // Player hand does not have at least 4 cards
    }

    if board.len() < 3 {
        // 3 because it allows for evaluation on flop-only flop-turn-only boards
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Board".to_string(),
            expected_count: 3,
        });
        // Board does not have at least 3 cards
    }

    let hand_combinations: Vec<Vec<Card>> = player_hand.iter().cloned().combinations(2).collect();
    let board_combinations: Vec<Vec<Card>> = board.iter().cloned().combinations(3).collect();

    let best_rank = hand_combinations
        .iter()
        .cartesian_product(board_combinations.iter())
        .map(|(hand, board)| {
            let mut all_cards = hand.clone();
            all_cards.extend(board.iter());
            high_evaluator::evaluate_hand(&all_cards)
        })
        .reduce(|acc, rank_res| {
            let acc = acc?;
            let rank = rank_res?;
            Ok(std::cmp::max(rank, acc))
        })
        .unwrap_or(Err(EvaluatorError::UnknownError(
            "No hand combos were evaluated".to_string(),
        )))?;

    Ok(best_rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trips_omaha() {
        let player_hand = Card::vec_from_str("AsKc9d7h").unwrap();
        let board = Card::vec_from_str("KhQsJdKdJs").unwrap();

        let player_rank = &evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_rank = player_rank
            .description
            .as_ref()
            .expect("Hand generated bad rank");
        assert_eq!("Trip Kings", string_rank);
    }

    #[test]
    fn not_a_flush_omaha() {
        let player_hand = Card::vec_from_str("AsQh2h5d").unwrap();
        let board = Card::vec_from_str("3s8sKs3dQs").unwrap();

        let player_rank = &evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        let string_rank = player_rank
            .description
            .as_ref()
            .expect("Hand generated bad rank");
        assert_eq!("Two Pair of Queens and 3s", string_rank);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn omaha_high_hands(b: &mut Bencher) {
        let player_hand = Card::vec_from_str("2s3c4h7cJhKs").unwrap();
        let board = Card::vec_from_str("5h6dAsTdTh").unwrap();
        b.iter(|| {
            let _player1_rank = evaluate_hand(&player_hand, &board).expect("Evaluation failed");
        })
    }
}
