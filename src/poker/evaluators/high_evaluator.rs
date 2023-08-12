use super::EvaluatorError;

use crate::core::{Card, Value};
use crate::poker::ranks::{BasicRank, HighRank};
use crate::poker::tables;
use std::collections::HashSet;
use std::num::Wrapping;
use std::ops::{Add, AddAssign, BitAnd, BitXor, BitXorAssign, Shl, Shr};

/// Evaluates the high hand for one player.
///
/// Returns a `HighRank`. If the total card count is not with the domain [5, 7], then an error will
/// return.
///
/// This implementation does not support the use of duplicate cards. If duplicate cards are found,
/// a `FailedToCalculateRank` error will return.
pub fn evaluate_hand(cards: &Vec<Card>) -> Result<HighRank, EvaluatorError> {
    let card_count = cards.len();
    if card_count < 5 {
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Set of cards".to_string(),
            expected_count: 5,
            actual_count: card_count as u64,
        });
    } else if card_count > 7 {
        return Err(EvaluatorError::TooManyCards {
            card_set_type: "Set of cards".to_string(),
            expected_count: 7,
            actual_count: card_count as u64,
        });
    }

    if card_count != HashSet::<&Card>::from_iter(cards.iter()).len() {
        return Err(EvaluatorError::FailedToCalculateRank(
            "Found duplicate cards".to_string(),
        ));
    }

    let cactus_kev_cards = Vec::from_iter(cards.iter().map(|card| card.calculate_bit_pattern()));

    let mut hand_results = Vec::new();

    for i0 in 0..cactus_kev_cards.len() {
        let c0 = cactus_kev_cards[i0];
        for i1 in i0 + 1..cactus_kev_cards.len() {
            let c1 = cactus_kev_cards[i1];
            for i2 in i1 + 1..cactus_kev_cards.len() {
                let c2 = cactus_kev_cards[i2];
                for i3 in i2 + 1..cactus_kev_cards.len() {
                    let c3 = cactus_kev_cards[i3];
                    for &c4 in cactus_kev_cards.iter().skip(i3 + 1) {
                        hand_results.push(eval_five_cards(c0, c1, c2, c3, c4));
                    }
                }
            }
        }
    }

    match hand_results
        .iter()
        .filter(|rank| rank.is_some())
        .flatten()
        .min()
    {
        None => Err(EvaluatorError::FailedToCalculateRank(
            "Cactus-Kev lookup tables could not find a valid rank entry".to_string(),
        )),
        Some(&best_rank) => {
            let mut hand_rank: u16 = 0;
            let mut sub_rank: u16 = 0;
            if best_rank >= 1 {
                let mut ranks_left = best_rank - 1;

                // distinct combos from high card to straight flush
                let strength_threshold = [1277, 2860, 858, 858, 10, 1277, 156, 156, 10];

                for (i, &subranks) in strength_threshold.iter().enumerate().rev() {
                    if ranks_left < subranks {
                        hand_rank = (i + 1) as u16;
                        sub_rank = subranks - ranks_left;
                        break;
                    }
                    ranks_left -= subranks;
                }
            }

            let rank = HighRank(BasicRank {
                strength: 7463 - best_rank as u32,
                hand_rank,
                sub_rank,
                description: Some(
                    get_string(hand_rank, sub_rank).unwrap_or_else(|err_str| err_str.to_string()),
                ),
            });
            Ok(rank)
        }
    }
}

fn get_string(hand_rank: u16, sub_rank: u16) -> Result<String, &'static str> {
    let hand_category;
    match hand_rank {
        1 => {
            hand_category = "High";

            if !(1..=1277).contains(&sub_rank) {
                return Err("Sub rank for high card was not valid");
            }

            let sub_str: &str;
            if sub_rank > 0 && sub_rank <= 4 {
                sub_str = "7";
            } else if sub_rank > 4 && sub_rank <= 18 {
                sub_str = "8";
            } else if sub_rank > 18 && sub_rank <= 52 {
                sub_str = "9";
            } else if sub_rank > 52 && sub_rank <= 121 {
                sub_str = "10";
            } else if sub_rank > 121 && sub_rank <= 246 {
                sub_str = "Jack";
            } else if sub_rank > 246 && sub_rank <= 455 {
                sub_str = "Queen";
            } else if sub_rank > 455 && sub_rank <= 784 {
                sub_str = "King";
            } else if sub_rank > 784 && sub_rank <= 1277 {
                sub_str = "Ace";
            } else {
                return Err("Sub rank for high card was not valid");
            }

            Ok(Vec::from([sub_str.to_owned(), hand_category.to_owned()]).join(" "))
        }
        2 => {
            hand_category = "Pair";

            let sub_str = match Value::from_int((sub_rank - 1) / 220) {
                Some(val) => val.get_readable_string() + "s",
                None => {
                    return Err("Sub rank for one pair was not valid");
                }
            };

            Ok(Vec::from([
                hand_category.to_owned(),
                "of".to_owned(),
                sub_str.to_owned(),
            ])
            .join(" "))
        }
        3 => {
            hand_category = "Two Pair";

            let first_pair_rank =
                (((2 * (sub_rank - 1) / 11) as f64 + 0.25).sqrt() - 0.5).floor() as u16 + 1;
            let sec_pair_kick_rank = sub_rank - (first_pair_rank - 1) * first_pair_rank / 2 * 11;

            let sub_str = match (
                Value::from_int(first_pair_rank),
                Value::from_int((sec_pair_kick_rank - 1) / 11),
            ) {
                (Some(first_pair), Some(sec_pair)) => Vec::from([
                    first_pair.get_readable_string() + "s",
                    "and".to_string(),
                    sec_pair.get_readable_string() + "s",
                ])
                .join(" "),
                _ => {
                    return Err("Sub rank for two pair was not valid");
                }
            };

            Ok(Vec::from([hand_category.to_owned(), "of".to_string(), sub_str]).join(" "))
        }
        4 => {
            hand_category = "Trip";

            let sub_str = match Value::from_int((sub_rank - 1) / 66) {
                Some(val) => val.get_readable_string() + "s",
                None => {
                    return Err("Sub rank for three of a kind was not valid");
                }
            };

            Ok(Vec::from([hand_category.to_owned(), sub_str.to_owned()]).join(" "))
        }
        5 => {
            hand_category = "Straight";

            if !(1..=10).contains(&sub_rank) {
                return Err("Sub rank for straight was not valid");
            }

            let sub_str = Value::from_int(sub_rank + 2).unwrap().get_readable_string();

            Ok(Vec::from([
                sub_str.to_owned(),
                "High".to_string(),
                hand_category.to_owned(),
            ])
            .join(" "))
        }
        6 => {
            hand_category = "Flush";

            let sub_str: &str;
            if sub_rank > 0 && sub_rank <= 4 {
                sub_str = "7";
            } else if sub_rank > 4 && sub_rank <= 18 {
                sub_str = "8";
            } else if sub_rank > 18 && sub_rank <= 52 {
                sub_str = "9";
            } else if sub_rank > 52 && sub_rank <= 121 {
                sub_str = "10";
            } else if sub_rank > 121 && sub_rank <= 246 {
                sub_str = "Jack";
            } else if sub_rank > 246 && sub_rank <= 455 {
                sub_str = "Queen";
            } else if sub_rank > 455 && sub_rank <= 784 {
                sub_str = "King";
            } else if sub_rank > 784 && sub_rank <= 1277 {
                sub_str = "Ace";
            } else {
                return Err("Sub rank for flush was not valid");
            }

            Ok(Vec::from([
                sub_str.to_owned(),
                "High".to_string(),
                hand_category.to_owned(),
            ])
            .join(" "))
        }
        7 => {
            // Full house

            let trip_rank = (sub_rank - 1) / 12;
            let mut pair_rank = (sub_rank - 1) % 12;

            if pair_rank >= trip_rank {
                pair_rank += 1;
            }

            match (Value::from_int(trip_rank), Value::from_int(pair_rank)) {
                (Some(trip_val), Some(pair_val)) => Ok(Vec::from([
                    trip_val.get_readable_string() + "s",
                    "Full of".to_string(),
                    pair_val.get_readable_string() + "s",
                ])
                .join(" ")),
                _ => Err("Sub rank for full house was not valid"),
            }
        }
        8 => {
            hand_category = "Quad";

            let sub_str = match Value::from_int((sub_rank - 1) / 12) {
                Some(val) => val.get_readable_string() + "s",
                None => {
                    return Err("Sub rank for four of a kind was not valid");
                }
            };

            Ok(Vec::from([hand_category.to_owned(), sub_str.to_owned()]).join(" "))
        }
        9 => {
            hand_category = "Straight Flush";

            if !(1..=10).contains(&sub_rank) {
                return Err("Sub rank for straight was not valid");
            }

            let sub_str = Value::from_int(sub_rank + 2).unwrap().get_readable_string();

            Ok(Vec::from([
                sub_str.to_owned(),
                "High".to_string(),
                hand_category.to_owned(),
            ])
            .join(" "))
        }
        _ => Err("Hand rank did not have a valid hand category"),
    }
}

fn eval_five_cards(c0: u32, c1: u32, c2: u32, c3: u32, c4: u32) -> Option<u16> {
    let q = (c0 | c1 | c2 | c3 | c4) >> 16;

    if c0 & c1 & c2 & c3 & c4 & 0xf000 != 0 {
        tables::FLUSHES.get(&q).cloned()
    } else if tables::UNIQUE5.contains_key(&q) {
        tables::UNIQUE5.get(&q).cloned()
    } else {
        let q = (c0 & 0xff) * (c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff);
        tables::HASH_VALUES.get(find_fast(Wrapping(q))).cloned()
    }
}

fn find_fast(mut query: Wrapping<u32>) -> usize {
    query.add_assign(Wrapping(0xe91aaa35));
    query.bitxor_assign(query.shr(16));
    query.add_assign(query.shl(8));
    query.bitxor_assign(query.shr(4));
    let b: Wrapping<u32> = query.shr(8).bitand(Wrapping(0x1ff));
    let a: Wrapping<u32> = query.add(query.shl(2)).shr(19);

    a.bitxor(Wrapping::<u32>(tables::HASH_ADJUST[b.0 as usize] as u32))
        .0 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threes_full_of_deuces_six_cards() {
        let player_hand = Vec::from([Card::from(1), Card::from(2)]);
        let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);

        let mut all_cards = player_hand.clone();
        all_cards.extend(board.iter());

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

        assert_eq!(6, player2_rank.hand_rank);
        assert_eq!(1, player2_rank.sub_rank);

        assert_eq!(player1_rank, player2_rank);
    }

    #[test]
    fn different_rank_by_1() {
        let player1_hand = Card::vec_from_str("2s3s4s5s8s").unwrap(); // stronger high hand
        let player2_hand = Card::vec_from_str("2h3h4h5h7h").unwrap();

        let player1_rank = evaluate_hand(&player1_hand).expect("Evaluation failed");
        let player2_rank = evaluate_hand(&player2_hand).expect("Evaluation failed");

        assert!(player1_rank > player2_rank);
    }

    #[test]
    fn cooler_holdem_example() {
        let board = Card::vec_from_str("2d9d2c9h3h").unwrap();
        let mut player1_hand = Card::vec_from_str("8h9s").unwrap();
        let mut player2_hand = Card::vec_from_str("9c3s").unwrap();

        player1_hand.extend(board.iter());
        player2_hand.extend(board.iter());

        let player1_rank = evaluate_hand(&player1_hand).expect("Evaluation failed");
        let player2_rank = evaluate_hand(&player2_hand).expect("Evaluation failed");

        assert_eq!(
            player1_rank
                .description
                .as_ref()
                .expect("Player 1 hand generated bad rank"),
            "9s Full of 2s"
        );
        assert_eq!(
            player2_rank
                .description
                .as_ref()
                .expect("Player 2 hand generated bad rank"),
            "9s Full of 3s"
        );
        assert!(player1_rank < player2_rank);
    }

    #[test]
    fn string_pairs_two_pairs_highs() {
        let hands = vec![
            ("2c2h4c5s7s", "Pair of 2s"),
            ("2c2hAcKsQs", "Pair of 2s"),
            ("3c3hAcKsQs", "Pair of 3s"),
            ("7c7hAcKsJs", "Pair of 7s"),
            ("2c2hAcQsQd", "Two Pair of Queens and 2s"),
            ("2c7hAcQcQs", "Pair of Queens"),
            ("2c7hTcKsQs", "King High"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand)
                .expect(&format!("Evaluation failed for hand {:?}", player_hand));

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_trips() {
        let hands = vec![
            ("2c2h2s3s4s", "Trip 2s"),
            ("2c2h2sAsKs", "Trip 2s"),
            ("3c3hAc3sKs", "Trip 3s"),
            ("4c4h4s2s3s", "Trip 4s"),
            ("AcAhAsKsQs", "Trip Aces"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straights() {
        let hands = vec![
            ("As2c3c4d5h", "5 High Straight"),
            ("2s3c4c5d6h", "6 High Straight"),
            ("3s4c5c6d7h", "7 High Straight"),
            ("4s5c6c7d8h", "8 High Straight"),
            ("5s6c7c8d9h", "9 High Straight"),
            ("6s7c8c9dTh", "10 High Straight"),
            ("7s8c9cTdJh", "Jack High Straight"),
            ("8s9cTcJdQh", "Queen High Straight"),
            ("9sTcJcQdKh", "King High Straight"),
            ("TsJcQcKdAh", "Ace High Straight"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_flushes() {
        let hands = vec![
            ("2s3s4s5s7s", "7 High Flush"),
            ("AsKsQsJs9s", "Ace High Flush"),
            ("As2s3s4s6s", "Ace High Flush"),
            ("3h6h9h5hTh", "10 High Flush"),
            ("5d9dJdQdKd", "King High Flush"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_boats() {
        let hands = vec![
            ("2s2c2h3d3s", "2s Full of 3s"),
            ("3s3c3h2d2s", "3s Full of 2s"),
            ("AsAcAhKdKs", "Aces Full of Kings"),
            ("2s2c2hAdAs", "2s Full of Aces"),
            ("5s5c5hTdTs", "5s Full of 10s"),
            ("5s5c5d4d4s", "5s Full of 4s"),
            ("5s5c5d6d6s", "5s Full of 6s"),
            ("6s6c6d5d5s", "6s Full of 5s"),
            ("6s6c6d7d7s", "6s Full of 7s"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_quads() {
        let hands = vec![
            ("2s2c2h2d3d", "Quad 2s"),
            ("AsAcAhAdKd", "Quad Aces"),
            ("QsQcQhQd4d", "Quad Queens"),
            ("7s7c7h7d6d", "Quad 7s"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn string_straight_flushes() {
        let hands = vec![
            ("As2s3s4s5s", "5 High Straight Flush"),
            ("2s3s4s5s6s", "6 High Straight Flush"),
            ("3d4d5d6d7d", "7 High Straight Flush"),
            ("4h5h6h7h8h", "8 High Straight Flush"),
            ("5c6c7c8c9c", "9 High Straight Flush"),
            ("6s7s8s9sTs", "10 High Straight Flush"),
            ("7h8h9hThJh", "Jack High Straight Flush"),
            ("8c9cTcJcQc", "Queen High Straight Flush"),
            ("9dTdJdQdKd", "King High Straight Flush"),
            ("TsJsQsKsAs", "Ace High Straight Flush"),
        ];
        for (h, expected_str) in hands {
            let player_hand = Card::vec_from_str(h).unwrap();

            let player_rank = evaluate_hand(&player_hand).expect("Evaluation failed");

            let string_rank = player_rank
                .description
                .as_ref()
                .expect("Hand generated bad rank");
            assert_eq!(expected_str, string_rank, "\nFailed on hand {}\n", h);
        }
    }

    #[test]
    fn duplicate_cards_two_pair() {
        let player_hand = Card::vec_from_str("5h2dAdAs5h").unwrap();

        let player_rank =
            evaluate_hand(&player_hand).expect_err("Set of cards with duplicates has a valid rank");

        assert_eq!(
            player_rank,
            EvaluatorError::FailedToCalculateRank("Found duplicate cards".to_string())
        );
    }

    #[test]
    fn duplicate_cards_flush() {
        let player_hand = Card::vec_from_str("5h2hAhQh5h").unwrap();

        let player_rank =
            evaluate_hand(&player_hand).expect_err("Evaluator was able to calculate rank");

        assert_eq!(
            player_rank,
            EvaluatorError::FailedToCalculateRank("Found duplicate cards".to_string())
        );

        // If the duplicate gaurd did not exist, then the evaluator would output the following
        // error:
        // assert_eq!(player_rank, EvaluatorError::FailedToCalculateRank("Cactus-Kev lookup tables could not find a valid rank entry".to_string()));
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

            assert_eq!(6, player1_rank.rank_strength >> 12);
            assert_eq!(1, player1_rank.rank_strength & 0xFFF);

            assert_eq!(player1_rank, player2_rank);
        })
    }
}
