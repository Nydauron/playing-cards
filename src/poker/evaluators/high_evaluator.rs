use super::Evaluator;
use super::LOOKUP_TABLE;

use crate::core::Card;

pub struct HighEvaluator;

impl Evaluator for HighEvaluator {
    fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Vec<u64>, &str> {
        let card_count = player_hand.len() + board.len();
        if card_count < 5 || card_count > 7 {
            return Err("Provided number of cards does not fall inclusively between 5 and 7");
        }
        let mut all_cards = player_hand.to_owned();
        
        all_cards.extend(board.to_owned());

        let mut rank = 53;
        
        for c in all_cards {
            rank = LOOKUP_TABLE[(rank + card_to_int(&c)) as usize];
        }

        if card_count < 7 {
            Ok(Vec::from([LOOKUP_TABLE[(rank) as usize] as u64]))
        } else {
            Ok(Vec::from([rank as u64]))
        }
    }
}

fn card_to_int(c: &Card) -> i32 {
    ((c.value as i32) * 4) + (c.suit as i32) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use super::super::init_lookup_table;

    #[test]
    fn threes_full_of_deuces_six_cards() {
        init_lookup_table();
        let player_hand = Vec::from([Card::from(1), Card::from(2)]);
        let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);

        let eval = HighEvaluator{};
        
        let rank = eval.evaluate_hand(&player_hand, &board).expect("Evaluation failed")[0];

        assert_eq!(7, rank >> 12);
        assert_eq!(13, rank & 0xFFF);
    }

    #[test]
    fn same_rank_different_cards() {
        init_lookup_table();
        let player1_hand = Card::vec_from_str("2s3s4s5s7s").unwrap();
        let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap();

        let eval = HighEvaluator{};
        
        let player1_rank = eval.evaluate_hand(&player1_hand, &Vec::new()).expect("Evaluation failed")[0];
        let player2_rank = eval.evaluate_hand(&player2_hand, &Vec::new()).expect("Evaluation failed")[0];

        assert_eq!(6, player1_rank >> 12);
        assert_eq!(1, player1_rank & 0xFFF);
        
        assert_eq!(player1_rank, player2_rank);
    }

    #[bench]
    fn bench_same_rank_different_cards(b: &mut Bencher) {
        init_lookup_table();
        b.iter(|| {
            let player1_hand = Card::vec_from_str("2s3s4s5s7s").unwrap();
            let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap();

            let eval = HighEvaluator{};
            
            let player1_rank = eval.evaluate_hand(&player1_hand, &Vec::new()).expect("Evaluation failed")[0];
            let player2_rank = eval.evaluate_hand(&player2_hand, &Vec::new()).expect("Evaluation failed")[0];

            assert_eq!(6, player1_rank >> 12);
            assert_eq!(1, player1_rank & 0xFFF);
            
            assert_eq!(player1_rank, player2_rank);
        })
    }

    #[test]
    fn string_pairs_two_pairs_highs() {
        let hands = vec![("2c2h4c5s7s", "Pair of 2s"), ("2c2hAcKsQs", "Pair of 2s"), ("3c3hAcKsQs", "Pair of 3s"), ("7c7hAcKsJs", "Pair of 7s"), ("2c2hAcQsQs", "Two Pair of Queens and 2s"), ("2c7hAcQsQs", "Pair of Queens"), ("2c7hTcKsQs", "King High")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_trips() {
        let hands = vec![("2c2h2s3s4s", "Trip 2s"), ("2c2h2sAsKs", "Trip 2s"), ("3c3hAc3sKs", "Trip 3s"), ("4c4h4s2s3s", "Trip 4s"), ("AcAhAsKsQs", "Trip Aces")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straights() {
        let hands = vec![("As2c3c4d5h", "5 High Straight"), ("2s3c4c5d6h", "6 High Straight"), ("3s4c5c6d7h", "7 High Straight"), ("4s5c6c7d8h", "8 High Straight"), ("5s6c7c8d9h", "9 High Straight"), ("6s7c8c9dTh", "10 High Straight"), ("7s8c9cTdJh", "Jack High Straight"), ("8s9cTcJdQh", "Queen High Straight"), ("9sTcJcQdKh", "King High Straight"), ("TsJcQcKdAh", "Ace High Straight")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_flushes() {
        let hands = vec![("2s3s4s5s7s", "7 High Flush"), ("AsKsQsJs9s", "Ace High Flush"), ("As2s3s4s6s", "Ace High Flush"), ("3h6h9h5hTh", "10 High Flush"), ("5d9dJdQdKd", "King High Flush")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_boats() {
        let hands = vec![("2s2c2h3d3s", "2s Full of 3s"), ("3s3c3h2d2s", "3s Full of 2s"), ("AsAcAhKdKs", "Aces Full of Kings"), ("2s2c2hAdAs", "2s Full of Aces"), ("5s5c5hTdTs", "5s Full of 10s"), ("5s5c5d4d4s", "5s Full of 4s"), ("5s5c5d6d6s", "5s Full of 6s"), ("6s6c6d5d5s", "6s Full of 5s"), ("6s6c6d7d7s", "6s Full of 7s")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_quads() {
        let hands = vec![("2s2c2h2d3d", "Quad 2s"), ("AsAcAhAdKd", "Quad Aces"), ("QsQcQhQd4d", "Quad Queens"), ("7s7c7h7d6d", "Quad 7s")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straight_flushes() {
        let hands = vec![("As2s3s4s5s", "5 High Straight Flush"), ("2s3s4s5s6s", "6 High Straight Flush"), ("3d4d5d6d7d", "7 High Straight Flush"), ("4h5h6h7h8h", "8 High Straight Flush"), ("5c6c7c8c9c", "9 High Straight Flush"), ("6s7s8s9sTs", "10 High Straight Flush"), ("7h8h9hThJh", "Jack High Straight Flush"), ("8c9cTcJcQc", "Queen High Straight Flush"), ("9dTdJdQdKd", "King High Straight Flush"), ("TsJsQsKsAs", "Ace High Straight Flush")];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let eval = HighEvaluator{};

            let player_rank = eval.evaluate_hand(&player_hand, &Vec::new()).expect("Evaluation failed")[0];

            let string_rank = eval.get_string(player_rank).expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }
}
