use std::collections::HashMap;
use num_traits::FromPrimitive;

use itertools::Itertools;

use crate::{poker::ranks::{BasicRank, IntoRankStrengthIterator, RankStrengthIterator, BadugiRank}, core::{Card, Value}};

use super::EvaluatorError;

fn choose(n: u64, k: u64) -> u64 {
    if k == 0 {
        return 1
    }
    n * choose(n - 1, k - 1) / k
}

pub fn evaluate_hand(player_hand: &Vec<Card>) -> Result<BadugiRank, EvaluatorError> {
    if player_hand.len() > 4 {
        return Err(EvaluatorError::TooManyCards("The player hand had too many cards".to_string(), 4));
    }
    if player_hand.len() < 4 {
        return Err(EvaluatorError::NotEnoughCards("The player hand did not have enough cards".to_string(), 4));
    }
    let mut suit_bits = 0;
    let mut rank_bits = 0;
    for c in player_hand {
        suit_bits |= (c.calculate_bit_pattern() >> 12) & 0xf;
        rank_bits |= (c.calculate_bit_pattern() >> 16) & 0x1fff;
    }
    let mut best_hand_card_count = 0;

    while suit_bits != 0 && rank_bits != 0 {
        suit_bits &= suit_bits - 1;
        rank_bits &= rank_bits - 1;

        best_hand_card_count += 1;
    }
    
    player_hand.iter().combinations(best_hand_card_count)
        .filter(|canidate_hand| {
            let mut suit_bits = 0;
            let mut rank_bits = 0;
            for c in canidate_hand {
                suit_bits |= (c.calculate_bit_pattern() >> 12) & 0xf;
                rank_bits |= (c.calculate_bit_pattern() >> 16) & 0x1fff;
            }
            let mut distinct_rank_suit_cards = 0;

            while suit_bits != 0 && rank_bits != 0 {
                suit_bits &= suit_bits - 1;
                rank_bits &= rank_bits - 1;

                distinct_rank_suit_cards += 1;
            }

            distinct_rank_suit_cards == best_hand_card_count
        })
        .map(|canidate_hand| {
            let card_ranks = canidate_hand.iter()
                .map(|&card| {
                    (card.value.clone() as u8 + 1) % 13
                })
                .sorted_by(|a,b | {
                    b.cmp(a)
                })
                .collect::<Vec<_>>();

            let mut base_strength = 1;
            let card_count = card_ranks.len();

            for i in 1..card_count {
                base_strength += choose(13, i as u64);
            }

            let (_, rank) = card_ranks.iter()
                .enumerate()
                .fold((13, BasicRank{strength: base_strength as u32, hand_rank: card_count as u16, sub_rank: 1, description: None}),
                    |(prev_rank_strength, mut acc), (i, rank_strength)| {
                        if acc.description.is_none() {
                            let hand_name_mapping: HashMap<usize, &str> = HashMap::from([
                                (1, "1-card hand"),
                                (2, "2-card hand"),
                                (3, "3-card hand"),
                                (4, "Badugi"),
                            ]);
                            let value_str: String = Value::from_u8((rank_strength - 1) % 13).map_or("".to_string(), |v| {
                                format!("{}-high ", v.get_readable_string())
                            });
                            acc.description = Some(format!("{}{}", value_str, hand_name_mapping[&card_count]));
                        }
                        for s in (rank_strength + 1)..prev_rank_strength {
                            let strength_inc = choose((s - 1) as u64, (card_count - i - 1) as u64);
                            acc.strength += strength_inc as u32;
                            acc.sub_rank += strength_inc as u16;
                        }

                        (*rank_strength, acc)
                    });

            BadugiRank(rank)
        })
        .fold(Err(EvaluatorError::UnknownError("No valid rank was generated".to_string())), |acc, rank| {
            if let Ok(acc) = acc {
                if rank > acc {
                    Ok(rank)
                } else {
                    Ok(acc)
                }
            } else {
                Ok(rank)
            }
         })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_all_same_suit() {
        let hand = Card::vec_from_str("2h4hThQh").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        let expected_rank  = BadugiRank(
            BasicRank {
                strength: 12,
                hand_rank: 1,
                sub_rank: 12,
                description: Some("2-high 1-card hand".to_string()),
            }
        );
        assert_eq!(rank, expected_rank);
    }

    #[test]
    fn hand_all_same_rank() {
        let hand = Card::vec_from_str("QhQsQdQc").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        let expected_rank = BadugiRank(
            BasicRank {
                strength: 2,
                hand_rank: 1,
                sub_rank: 2,
                description: Some("Queen-high 1-card hand".to_string()),
            }
        );
        assert_eq!(expected_rank, rank);
    }

    #[test]
    fn card_hand_size_2() {
        let hand = Card::vec_from_str("2h4hTd2d").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        // + 1 since all ranks start at strength of 1
        // +13 to account for all hand combos with only 1 card
        // +63 for Σ nCr(n - 1, 1) for all n ∈ [4, 13)
        // + 1 for Σ nCr(n - 1, 0) for all n ∈ [2, 3)
        let expected_rank = BadugiRank(
            BasicRank {
                strength: 1 + 13 + 63 + 1,
                hand_rank: 2,
                sub_rank: 65,
                description: Some("4-high 2-card hand".to_string()),
            }
        );
        assert_eq!(rank, expected_rank);
    }

    #[test]
    fn card_hand_size_3() {
        let hand = Card::vec_from_str("3d7h6s7c").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        // +  1 since all ranks start at strength of 1
        // + 91 to account for all hand combos with only 1 or 2 cards
        // +200 for Σ nCr(n - 1, 2) for all n ∈ [7, 13)
        // +  0 for Σ nCr(n - 1, 1) for all n ∈ [6, 6) but |n| = 0
        // +  2 for Σ nCr(n - 1, 0) for all n ∈ [3, 5)
        let expected_rank = BadugiRank(
            BasicRank {
            strength: 1 + 91 + 200 + 0 + 2,
            hand_rank: 3,
            sub_rank: 203,
            description: Some("7-high 3-card hand".to_string()),
        }
        );
        assert_eq!(expected_rank, rank)
    }

    #[test]
    fn badugi_hand() {
        let hand = Card::vec_from_str("As3dKc5h").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        // +  1 since all ranks start at strength of 1
        // +377 to account for all hand combos with only 1-3 cards
        // +  0 for Σ nCr(n - 1, 3) for all n ∈ [13, 13) but |n| = 0
        // +161 for Σ nCr(n - 1, 2) for all n ∈ [5, 12)
        // +  2 for Σ nCr(n - 1, 1) for all n ∈ [3, 4)
        // +  1 for Σ nCr(n - 1, 0) for all n ∈ [1, 2)
        let expected_rank = BadugiRank(
            BasicRank {
                strength: 1 + 377 + 0 + 161 + 2 + 1,
                hand_rank: 4,
                sub_rank: 165,
                description: Some("King-high Badugi".to_string()),
            }
        );
        assert_eq!(expected_rank, rank);
    }

    #[test]
    fn budugi_hand_10th_best() {
        let hand = Card::vec_from_str("As2d5c6h").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        // +  1 since all ranks start at strength of 1
        // +377 to account for all hand combos with only 1-3 cards
        // +490 for Σ nCr(n - 1, 3) for all n ∈ [6, 13)
        // +  0 for Σ nCr(n - 1, 2) for all n ∈ [5, 5) but |n| = 0
        // +  3 for Σ nCr(n - 1, 1) for all n ∈ [2, 4)
        // +  0 for Σ nCr(n - 1, 0) for all n ∈ [1, 1) but |n| = 0
        let expected_rank = BadugiRank(
            BasicRank {
                strength: 1 + 377 + 490 + 0 + 3 + 0,
                hand_rank: 4,
                sub_rank: 494,
                description: Some("6-high Badugi".to_string()),
            }
        );
        assert_eq!(expected_rank, rank);
    }

    #[test]
    fn best_badugi_hand() {
        let hand = Card::vec_from_str("As2d3c4h").expect("Cards did not parse correctly");
        let rank = evaluate_hand(&hand).expect("Hand did not evaluate correctly");

        // +  1 since all ranks start at strength of 1
        // +377 to account for all hand combos with only 1-3 cards
        // +495 for Σ nCr(n - 1, 3) for all n ∈ [4, 13)
        // +  0 for Σ nCr(n - 1, 2) for all n ∈ [5, 5) but |n| = 0
        // +  0 for Σ nCr(n - 1, 1) for all n ∈ [2, 4) but |n| = 0
        // +  0 for Σ nCr(n - 1, 0) for all n ∈ [1, 1) but |n| = 0
        let expected_rank = BadugiRank(
            BasicRank {
                strength: 1 + 377 + 495 + 0 + 0 + 0,
                hand_rank: 4,
                sub_rank: 496,
                description: Some("4-high Badugi".to_string()),
            }
        );
        assert_eq!(expected_rank, rank);
    }
}
