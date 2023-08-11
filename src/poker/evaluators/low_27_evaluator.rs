use std::ops::Deref;

use super::{high_evaluator, EvaluatorError};

use crate::poker::evaluator_result::{RankStrengthIterator, IntoRankStrengthIterator};
use crate::poker::rank::BasicRank;
use crate::core::Card;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Low27Rank(pub BasicRank);

impl Ord for Low27Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.strength.cmp(&other.0.strength)
    }
}

impl Deref for Low27Rank {
    type Target = BasicRank;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoRankStrengthIterator for Low27Rank {
    fn into_strength_iter(self) -> RankStrengthIterator {
        RankStrengthIterator::from((*self).strength)
    }
}

/// Evaluates the low hand for one player.
///
/// Returns a `Vec<Rank>`. If the total card count is not with the domain [5, 7], then an error
/// will return.
pub fn evaluate_hand(cards: &Vec<Card>) -> Result<Low27Rank, EvaluatorError> {
    high_evaluator::evaluate_hand(cards).and_then(|high_rank| {
        let mut rank = (*high_rank).clone();
        rank.strength = 7463 - rank.strength;
        Ok(Low27Rank(rank))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threes_full_of_deuces_six_cards() {
        let player_hand = Vec::from([Card::from(1), Card::from(2)]);
        let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);

        let mut all_cards = player_hand.clone();
        all_cards.extend(board);
        let rank = evaluate_hand(&all_cards).expect("Evaluation failed");

        assert_eq!(7, rank.hand_rank);
        assert_eq!(13, rank.sub_rank);
    }

    #[test]
    fn same_rank_different_cards() {
        let player1_hand = Card::vec_from_str("2s3s4s5s7s").unwrap();
        let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap();

        let player1_rank = evaluate_hand(&player1_hand).expect("Evaluation failed");
        let player2_rank = evaluate_hand(&player2_hand).expect("Evaluation failed");

        assert_eq!(6, player1_rank.hand_rank);
        assert_eq!(1, player1_rank.sub_rank);
        
        assert_eq!(player1_rank, player2_rank);
    }

    #[test]
    fn different_rank_by_1() {
        let player1_hand = Card::vec_from_str("2s3s4s5s8s").unwrap();
        let player2_hand = Card::vec_from_str("2h4h5h6h7h").unwrap(); // stronger low hand

        let player1_rank = &evaluate_hand(&player1_hand).expect("Evaluation failed");
        let player2_rank = &evaluate_hand(&player2_hand).expect("Evaluation failed");
        
        assert!(player2_rank > player1_rank);
    }

    #[test]
    fn cooler_2to7_example_1() {
        let player1_hand = Card::vec_from_str("5h3d7h2s9c").unwrap();
        let player2_hand = Card::vec_from_str("4s3c2h6s8s").unwrap();

        let player1_rank = evaluate_hand(&player1_hand).expect("Evaluation failed");
        let player2_rank = evaluate_hand(&player2_hand).expect("Evaluation failed");

        assert_eq!(player1_rank.description.as_ref().expect("Player 1 hand generated bad rank"), "9 High");
        assert_eq!(player2_rank.description.as_ref().expect("Player 2 hand generated bad rank"), "8 High");
        assert!(player1_rank < player2_rank);
    }

    #[test]
    fn cooler_2to7_example_2() {
        let player1_hand = Card::vec_from_str("5h3d7h2s8c").unwrap();
        let player2_hand = Card::vec_from_str("4s3c2h6s8s").unwrap();

        let player1_rank = evaluate_hand(&player1_hand).expect("Evaluation failed");
        let player2_rank = evaluate_hand(&player2_hand).expect("Evaluation failed");

        assert_eq!(player1_rank.description.as_ref().expect("Player 1 hand generated bad rank"), "8 High");
        assert_eq!(player2_rank.description.as_ref().expect("Player 2 hand generated bad rank"), "8 High");
        assert!(player1_rank < player2_rank);
    }
    #[test]
    fn string_pairs_two_pairs_highs() {
        let hands = vec![("2c2h4c5s7s", "Pair of 2s"), ("2c2hAcKsQs", "Pair of 2s"), ("3c3hAcKsQs", "Pair of 3s"), ("7c7hAcKsJs", "Pair of 7s"), ("2c2hAcQsQs", "Two Pair of Queens and 2s"), ("2c7hAcQsQs", "Pair of Queens"), ("2c7hTcKsQs", "King High")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_trips() {
        let hands = vec![("2c2h2s3s4s", "Trip 2s"), ("2c2h2sAsKs", "Trip 2s"), ("3c3hAc3sKs", "Trip 3s"), ("4c4h4s2s3s", "Trip 4s"), ("AcAhAsKsQs", "Trip Aces")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straights() {
        let hands = vec![("As2c3c4d5h", "5 High Straight"), ("2s3c4c5d6h", "6 High Straight"), ("3s4c5c6d7h", "7 High Straight"), ("4s5c6c7d8h", "8 High Straight"), ("5s6c7c8d9h", "9 High Straight"), ("6s7c8c9dTh", "10 High Straight"), ("7s8c9cTdJh", "Jack High Straight"), ("8s9cTcJdQh", "Queen High Straight"), ("9sTcJcQdKh", "King High Straight"), ("TsJcQcKdAh", "Ace High Straight")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_flushes() {
        let hands = vec![("2s3s4s5s7s", "7 High Flush"), ("AsKsQsJs9s", "Ace High Flush"), ("As2s3s4s6s", "Ace High Flush"), ("3h6h9h5hTh", "10 High Flush"), ("5d9dJdQdKd", "King High Flush")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_boats() {
        let hands = vec![("2s2c2h3d3s", "2s Full of 3s"), ("3s3c3h2d2s", "3s Full of 2s"), ("AsAcAhKdKs", "Aces Full of Kings"), ("2s2c2hAdAs", "2s Full of Aces"), ("5s5c5hTdTs", "5s Full of 10s"), ("5s5c5d4d4s", "5s Full of 4s"), ("5s5c5d6d6s", "5s Full of 6s"), ("6s6c6d5d5s", "6s Full of 5s"), ("6s6c6d7d7s", "6s Full of 7s")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_quads() {
        let hands = vec![("2s2c2h2d3d", "Quad 2s"), ("AsAcAhAdKd", "Quad Aces"), ("QsQcQhQd4d", "Quad Queens"), ("7s7c7h7d6d", "Quad 7s")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straight_flushes() {
        let hands = vec![("As2s3s4s5s", "5 High Straight Flush"), ("2s3s4s5s6s", "6 High Straight Flush"), ("3d4d5d6d7d", "7 High Straight Flush"), ("4h5h6h7h8h", "8 High Straight Flush"), ("5c6c7c8c9c", "9 High Straight Flush"), ("6s7s8s9sTs", "10 High Straight Flush"), ("7h8h9hThJh", "Jack High Straight Flush"), ("8c9cTcJcQc", "Queen High Straight Flush"), ("9dTdJdQdKd", "King High Straight Flush"), ("TsJsQsKsAs", "Ace High Straight Flush")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank.description.as_ref().expect("Hand generated bad rank");
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

            let player1_rank = evaluate_hand(&player1_hand).expect("Evaluation failed");
            let player2_rank = evaluate_hand(&player2_hand).expect("Evaluation failed");

            assert_eq!(6, player1_rank.get_rank_strength() >> 12);
            assert_eq!(1, player1_rank.get_rank_strength() & 0xFFF);

            assert_eq!(player1_rank, player2_rank);
        })
    }
}
