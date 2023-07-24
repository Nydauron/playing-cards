use super::EvaluatorError;
use super::super::rank::Rank;

use itertools::Itertools;

use crate::core::Card;
use crate::poker::evaluators::high_evaluator;

/// Evaluates the Omaha high hand for one player.
///
/// Returns a `Vec<HighRank>` than can be compared directly against other `HighRank`s. If
/// the player's hand contains less than 4 cards or the board contains less than 3 cards,
/// then an error will return.
pub fn evaluate_hand(player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Rank>, EvaluatorError> {
    if player_hand.len() < 4 {
        return Err(EvaluatorError::NotEnoughCards("Player hand".to_string(), 4));
        // Player hand does not have at least 4 cards
    }

    if board.len() < 3 { // 3 because it allows for evaluation on flop-only flop-turn-only boards
        return Err(EvaluatorError::NotEnoughCards("Board".to_string(), 3));
        // Board does not have at least 3 cards
    }

    let hand_combinations: Vec<Vec<Card>> = player_hand.clone().into_iter().clone().combinations(2).collect();
    let board_combinations: Vec<Vec<Card>> = board.clone().into_iter().clone().combinations(3).collect();

    let result =
        hand_combinations
        .iter()
        .cartesian_product(board_combinations.iter())
        .map(|(hand, board)| {
            let rank_arr = high_evaluator::evaluate_hand(hand, board);
            match rank_arr {
                Err(e) => Err(e),
                Ok(rank_arr) => {
                    if rank_arr.len() != 1 {
                        Err(EvaluatorError::UnknownError("Rank array did not match expected length of 1".to_string()))
                    } else {
                        Ok(rank_arr[0].clone())
                    }
                }
            }
        })
        .reduce(|acc, rank_res| {
            match acc {
                Ok(acc) => {
                    match rank_res {
                        Ok(rank) => {
                            if rank > acc {
                                Ok(rank)
                            } else {
                                Ok(acc)
                            }
                        },
                        Err(_) => rank_res,
                    }
                },
                Err(_) => acc,
            }
        })
        .unwrap_or(Err(EvaluatorError::UnknownError("No hand combos were evaluated".to_string())));

    match result {
        Ok(rank) => {
            Ok(vec![rank])
        },
        Err(e) => {
            Err(e)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trips_omaha() {
        let player_hand = Card::vec_from_str("AsKc9d7h").unwrap();
        let board = Card::vec_from_str("KhQsJdKdJs").unwrap();

        let player_rank = &evaluate_hand(&player_hand, &board).expect("Evaluation failed")[0];

        let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
        assert_eq!("Trip Kings", string_rank);
    }

    #[test]
    fn not_a_flush_omaha() {
        let player_hand = Card::vec_from_str("AsQh2h5d").unwrap();
        let board = Card::vec_from_str("3s8sKs3dQs").unwrap();

        let player_rank = &evaluate_hand(&player_hand, &board).expect("Evaluation failed")[0];

        let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
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
