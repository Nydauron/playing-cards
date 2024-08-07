use super::EvaluatorError;

use itertools::Itertools;

use crate::core::Card;
use crate::poker::evaluators::high_evaluator;
use crate::poker::ranks::HighRank;

/// Evaluates the Omaha high hand for one player
///
/// Returns a `HighRank`. If the player's hand contains less than 4 cards or the board contains
/// less than 3 cards, then a `NotEnoughCards` error will return.
///
/// This implementation does not support the use of duplicate cards. If duplicate cards are found
/// when both the player's cards and the board are chained, a `FailedToCalculateRank` error will
/// return.
pub fn evaluate_hand(
    player_hand: &Vec<Card>,
    board: &Vec<Card>,
) -> Result<HighRank, EvaluatorError> {
    const MINIMUM_PLAYER_CARDS: usize = 4;
    const MINIMUM_BOARD_CARDS: usize = 3;
    if player_hand.len() < MINIMUM_PLAYER_CARDS {
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Player hand".to_string(),
            expected_count: MINIMUM_PLAYER_CARDS as u64,
            actual_count: player_hand.len() as u64,
        });
    }

    if board.len() < MINIMUM_BOARD_CARDS {
        // 3 because it allows for evaluation on flop-only flop-turn-only boards
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Board".to_string(),
            expected_count: MINIMUM_BOARD_CARDS as u64,
            actual_count: board.len() as u64,
        });
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
        .unwrap_or(Err(EvaluatorError::FailedToCalculateRank(
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

    #[test]
    fn duplicate_cards_flush() {
        let player_hand = Card::vec_from_str("4s3c5h2h").unwrap();
        let board = Card::vec_from_str("2d8h5hAhTc").unwrap();

        let player_rank =
            evaluate_hand(&player_hand, &board).expect_err("Evaluator was able to calculate rank");

        assert_eq!(
            player_rank,
            EvaluatorError::FailedToCalculateRank("Found duplicate cards".to_string())
        );

        // If the duplicate guard did not exist in high_evaluator::evaluate_hand, then the evaluator
        // would output the following error:
        // assert_eq!(player_rank, EvaluatorError::FailedToCalculateRank("Cactus-Kev lookup tables could not find a valid rank entry".to_string()));
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
