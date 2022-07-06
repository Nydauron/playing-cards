use async_std::task;

use super::{Evaluator, HighEvaluator, init_lookup_table};
use super::super::{Rank, LowRank};

use crate::core::Card;

/// The wrapper struct for the Low Evaluator.
pub struct LowEvaluator;

impl LowEvaluator {
    /// Creates a new `LowEvaluator`.
    /// 
    /// Initializes the lookup table if it isn't already.
    pub fn new() -> Self {
        task::spawn(async {
            init_lookup_table();
        });
        Self{}
    }
}

impl Evaluator for LowEvaluator {
    type Output = LowRank;
    /// Evaluates the low hand for one player.
    ///
    /// Returns a `LowRank` than can be compared against directly against other ranks. If the
    /// total card count is not with the domain [5, 7], then an error will return.
    ///
    /// Examples
    /// ```rust
    /// use playing_cards::{core::Card, poker::{Evaluator, LowEvaluator, Rank}};
    ///
    /// let hand = Card::vec_from_str("2dTd3s5sAc").unwrap();
    ///
    /// let eval = LowEvaluator::new();
    ///
    /// let rank = eval.evaluate_hand(&hand, &Vec::new()).unwrap();
    ///
    /// assert_eq!(rank.get_string().unwrap(), "Ace High");
    /// ```
    ///
    /// ```rust
    /// use playing_cards::{core::Card, poker::{Evaluator, LowEvaluator, Rank}};
    ///
    /// let hand = Card::vec_from_str("2c4dKs2dKd").unwrap();
    ///
    /// let eval = LowEvaluator::new();
    ///
    /// let rank = eval.evaluate_hand(&hand, &Vec::new()).unwrap();
    ///
    /// assert_eq!(rank.get_string().unwrap(), "Two Pair of Kings and 2s");
    /// ```
    ///
    /// ```rust
    /// use playing_cards::{core::Card, poker::{Evaluator, LowEvaluator, Rank}};
    ///
    /// let hero_hand = Card::vec_from_str("6h7h2s3cTd").unwrap();
    /// let villan_hand = Card::vec_from_str("2c3s4s5s6d").unwrap();
    /// let board = vec![];
    ///
    /// let eval = LowEvaluator::new();
    ///
    /// let hero_rank = eval.evaluate_hand(&hero_hand, &board).unwrap();
    /// let villan_rank = eval.evaluate_hand(&villan_hand, &board).unwrap();
    ///
    /// assert_eq!(hero_rank.get_string().unwrap(), "10 High");
    /// assert_eq!(villan_rank.get_string().unwrap(), "6 High Straight");
    ///
    /// assert!(hero_rank > villan_rank); // Hero's hand is better than the villan's
    /// ```
    fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Self::Output, &str> {
        HighEvaluator{}.evaluate_hand(player_hand, board).and_then(|high| {
            Ok(LowRank::new(high.get_rank_strength()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threes_full_of_deuces_six_cards() {
        let player_hand = Vec::from([Card::from(1), Card::from(2)]);
        let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);

        let eval = LowEvaluator::new();
        
        let rank = eval.evaluate_hand(&player_hand, &board).expect("Evaluation failed");

        assert_eq!(7, rank.get_rank_strength() >> 12);
        assert_eq!(13, rank.get_rank_strength() & 0xFFF);
    }

    #[test]
    fn same_rank_different_cards() {
        let player1_hand = Card::vec_from_str("2s3s4s5s7s").unwrap();
        let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap();

        let eval = LowEvaluator::new();
        
        let player1_rank = eval.evaluate_hand(&player1_hand, &Vec::new()).expect("Evaluation failed");
        let player2_rank = eval.evaluate_hand(&player2_hand, &Vec::new()).expect("Evaluation failed");

        assert_eq!(6, player1_rank.get_rank_strength() >> 12);
        assert_eq!(1, player1_rank.get_rank_strength() & 0xFFF);
        
        assert_eq!(player1_rank, player2_rank);
    }

    #[test]
    fn different_rank_by_1() {
        let player1_hand = Card::vec_from_str("2s3s4s5s8s").unwrap();
        let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap(); // stronger low hand

        let eval = LowEvaluator::new();
        
        let player1_rank = eval.evaluate_hand(&player1_hand, &Vec::new()).expect("Evaluation failed");
        let player2_rank = eval.evaluate_hand(&player2_hand, &Vec::new()).expect("Evaluation failed");
        
        assert!(player2_rank > player1_rank);
    }

    #[test]
    fn cooler_2to7_example_1() {
        let board = vec![];
        let player1_hand = Card::vec_from_str("5h3d7h2s9c").unwrap();
        let player2_hand = Card::vec_from_str("4s3c2h6s8s").unwrap();

        let eval = LowEvaluator::new();

        let player1_rank = eval.evaluate_hand(&player1_hand, &board).expect("Evaluation failed");
        let player2_rank = eval.evaluate_hand(&player2_hand, &board).expect("Evaluation failed");

        assert_eq!(player1_rank.get_string().expect("Player 1 hand generated bad rank"), "9 High");
        assert_eq!(player2_rank.get_string().expect("Player 2 hand generated bad rank"), "8 High");
        assert!(player1_rank < player2_rank);
    }

    #[test]
    fn cooler_2to7_example_2() {
        let board = vec![];
        let player1_hand = Card::vec_from_str("5h3d7h2s8c").unwrap();
        let player2_hand = Card::vec_from_str("4s3c2h6s8s").unwrap();

        let eval = LowEvaluator::new();

        let player1_rank = eval.evaluate_hand(&player1_hand, &board).expect("Evaluation failed");
        let player2_rank = eval.evaluate_hand(&player2_hand, &board).expect("Evaluation failed");

        assert_eq!(player1_rank.get_string().expect("Player 1 hand generated bad rank"), "8 High");
        assert_eq!(player2_rank.get_string().expect("Player 2 hand generated bad rank"), "8 High");
        assert!(player1_rank < player2_rank);
    }
    #[test]
    fn string_pairs_two_pairs_highs() {
        let hands = vec![("2c2h4c5s7s", "Pair of 2s"), ("2c2hAcKsQs", "Pair of 2s"), ("3c3hAcKsQs", "Pair of 3s"), ("7c7hAcKsJs", "Pair of 7s"), ("2c2hAcQsQs", "Two Pair of Queens and 2s"), ("2c7hAcQsQs", "Pair of Queens"), ("2c7hTcKsQs", "King High")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_trips() {
        let hands = vec![("2c2h2s3s4s", "Trip 2s"), ("2c2h2sAsKs", "Trip 2s"), ("3c3hAc3sKs", "Trip 3s"), ("4c4h4s2s3s", "Trip 4s"), ("AcAhAsKsQs", "Trip Aces")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straights() {
        let hands = vec![("As2c3c4d5h", "5 High Straight"), ("2s3c4c5d6h", "6 High Straight"), ("3s4c5c6d7h", "7 High Straight"), ("4s5c6c7d8h", "8 High Straight"), ("5s6c7c8d9h", "9 High Straight"), ("6s7c8c9dTh", "10 High Straight"), ("7s8c9cTdJh", "Jack High Straight"), ("8s9cTcJdQh", "Queen High Straight"), ("9sTcJcQdKh", "King High Straight"), ("TsJcQcKdAh", "Ace High Straight")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_flushes() {
        let hands = vec![("2s3s4s5s7s", "7 High Flush"), ("AsKsQsJs9s", "Ace High Flush"), ("As2s3s4s6s", "Ace High Flush"), ("3h6h9h5hTh", "10 High Flush"), ("5d9dJdQdKd", "King High Flush")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_boats() {
        let hands = vec![("2s2c2h3d3s", "2s Full of 3s"), ("3s3c3h2d2s", "3s Full of 2s"), ("AsAcAhKdKs", "Aces Full of Kings"), ("2s2c2hAdAs", "2s Full of Aces"), ("5s5c5hTdTs", "5s Full of 10s"), ("5s5c5d4d4s", "5s Full of 4s"), ("5s5c5d6d6s", "5s Full of 6s"), ("6s6c6d5d5s", "6s Full of 5s"), ("6s6c6d7d7s", "6s Full of 7s")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_quads() {
        let hands = vec![("2s2c2h2d3d", "Quad 2s"), ("AsAcAhAdKd", "Quad Aces"), ("QsQcQhQd4d", "Quad Queens"), ("7s7c7h7d6d", "Quad 7s")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straight_flushes() {
        let hands = vec![("As2s3s4s5s", "5 High Straight Flush"), ("2s3s4s5s6s", "6 High Straight Flush"), ("3d4d5d6d7d", "7 High Straight Flush"), ("4h5h6h7h8h", "8 High Straight Flush"), ("5c6c7c8c9c", "9 High Straight Flush"), ("6s7s8s9sTs", "10 High Straight Flush"), ("7h8h9hThJh", "Jack High Straight Flush"), ("8c9cTcJcQc", "Queen High Straight Flush"), ("9dTdJdQdKd", "King High Straight Flush"), ("TsJsQsKsAs", "Ace High Straight Flush")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = LowEvaluator::new();

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed");

            let string_rank = player_rank.get_string().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_same_rank_different_cards(b: &mut Bencher) {
        b.iter(|| {
            let player1_hand = Card::vec_from_str("2s3s4s5s7s").unwrap();
            let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap();

            let eval = LowEvaluator::new();

            let player1_rank = eval.evaluate_hand(&player1_hand, &Vec::new()).expect("Evaluation failed")[0];
            let player2_rank = eval.evaluate_hand(&player2_hand, &Vec::new()).expect("Evaluation failed")[0];

            assert_eq!(6, player1_rank >> 12);
            assert_eq!(1, player1_rank & 0xFFF);

            assert_eq!(player1_rank, player2_rank);
        })
    }
}
