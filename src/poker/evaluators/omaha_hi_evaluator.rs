use async_std::task;

use super::{Evaluator, init_lookup_table, LOOKUP_TABLE, EvaluatorError};
use super::super::{Rank, HighRank};

use itertools::Itertools;

use crate::core::Card;

/// An evaluator for Omaha high hands.
///
/// The evaluator requires that the player has at least 4 cards and the board has at least 3
/// cards. In Omaha and Omaha-varients, the player is required to use only 2 cards from their
/// hand and 3 from the board. This evaluator permutates through these combinations in parallel
/// with the help of map-reduce.
///
/// Some games that can make use of this evaluator include but are not limited to Omaha, Omaha 8
/// (Hi/Lo), Big O, and Drawmaha.
pub struct OmahaHighEvaluator;

impl OmahaHighEvaluator {
    /// Creates a new `HighEvaluator`.
    ///
    /// Initializes the lookup table if it isn't already.
    pub fn new() -> Self {
        task::spawn(async {
            init_lookup_table();
        });
        Self{}
    }
}

impl Evaluator for OmahaHighEvaluator {
    /// Evaluates the Omaha high hand for one player.
    ///
    /// Returns a `HighRank` than can be compared against directly against other `HighRank`s. If
    /// the player's hand contains less than 4 cards and the board contains less than 3 cards,
    /// then an error will return.
    ///
    /// Examples
    /// ```rust
    /// use playing_cards::{core::Card, poker::{Evaluator, OmahaHighEvaluator, Rank}};
    ///
    /// let hand = Card::vec_from_str("2cAsAcKc").unwrap();
    /// let board = Card::vec_from_str("Ks2sTd8h7d").unwrap();
    ///
    /// let eval = OmahaHighEvaluator::new();
    ///
    /// let rank = eval.evaluate_hand(&hand, &board).unwrap();
    ///
    /// // Notice: Even though we can Aces in our hand, we can only use 2 cards from out hand to
    /// // make the best hand (e.g. the K and the 2 pair with the board).
    /// assert_eq!(rank.get_string().unwrap(), "Two Pair of Kings and 2s");
    /// ```
    ///
    /// ```rust
    /// use playing_cards::{core::Card, poker::{Evaluator, OmahaHighEvaluator, Rank}};
    ///
    /// let hand = Card::vec_from_str("AcKhKsTd").unwrap();
    /// let board = Card::vec_from_str("Tc5c3s6cQc").unwrap();
    ///
    /// let eval = OmahaHighEvaluator::new();
    ///
    /// let rank = eval.evaluate_hand(&hand, &board).unwrap();
    ///
    /// // Notice: Even though we have the Ace of Clubs in out hand, we do not have a flush, as we
    /// // need another club within our hand.
    /// assert_eq!(rank.get_string().unwrap(), "Pair of Kings");
    /// ```
    ///
    /// ```rust
    /// use playing_cards::{core::Card, poker::{Evaluator, OmahaHighEvaluator, Rank}};
    ///
    /// let hero_hand = Card::vec_from_str("5s6c9s7c").unwrap();
    /// let villan_hand = Card::vec_from_str("AhKdAsTh").unwrap();
    /// let board = Card::vec_from_str("8hTcAdQs6s").unwrap();
    ///
    /// let eval = OmahaHighEvaluator::new();
    ///
    /// let hero_rank = eval.evaluate_hand(&hero_hand, &board).unwrap();
    /// let villan_rank = eval.evaluate_hand(&villan_hand, &board).unwrap();
    ///
    /// assert_eq!(hero_rank.get_string().unwrap(), "10 High Straight");
    /// assert_eq!(villan_rank.get_string().unwrap(), "Trip Aces");
    ///
    /// assert!(hero_rank > villan_rank); // Hero's hand is better than the villans's
    /// ```
    fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<Rank>, EvaluatorError> {
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

        // Trying to run rayon map-reduce on the combinations is not efficient because of how quick
        // using the lookup table is.
        Ok(Vec::from([Rank::High(hand_combinations.iter().map(|player_comb| {
            board_combinations.clone().into_iter().map(|mut board_comb| {
                board_comb.extend(player_comb.to_owned());
                let card_count = board_comb.len();

                let mut rank = 53;
            
                for c in board_comb {
                    rank = LOOKUP_TABLE[(rank + c.to_int()) as usize];
                }

                if card_count < 7 {
                    HighRank::new(LOOKUP_TABLE[rank as usize] as u64)
                } else {
                    HighRank::new(rank as u64)
                }
            })
            .reduce(|a, b| {
                if a > b {
                    a
                } else {
                    b
                }
            }).unwrap()
        })
        .reduce(|a, b| {
            if a > b {
                a
            } else {
                b
            }
        }).unwrap())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trips_omaha() {
        let player_hand = Card::vec_from_str("AsKc9d7h").unwrap();
        let board = Card::vec_from_str("KhQsJdKdJs").unwrap();

        let eval = OmahaHighEvaluator::new();

        let player_rank = eval.evaluate_hand(&player_hand, &board).expect("Evaluation failed")[0];

        let string_rank = player_rank.get_string().expect("Hand generated bad rank");
        assert_eq!("Trip Kings", string_rank);
    }

    #[test]
    fn not_a_flush_omaha() {
        let player_hand = Card::vec_from_str("AsQh2h5d").unwrap();
        let board = Card::vec_from_str("3s8sKs3dQs").unwrap();

        let eval = OmahaHighEvaluator::new();

        let player_rank = eval.evaluate_hand(&player_hand, &board).expect("Evaluation failed")[0];

        let string_rank = player_rank.get_string().expect("Hand generated bad rank");
        assert_eq!("Two Pair of Queens and 3s", string_rank);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn omaha_high_hands(b: &mut Bencher) {
        init_lookup_table();
        let player_hand = Card::vec_from_str("2s3c4h7cJhKs").unwrap();
        let board = Card::vec_from_str("5h6dAsTdTh").unwrap();
        b.iter(|| {
            let eval = OmahaHighEvaluator::new();

            let _player1_rank = eval.evaluate_hand(&player_hand, &board).expect("Evaluation failed");
        })
    }
}
