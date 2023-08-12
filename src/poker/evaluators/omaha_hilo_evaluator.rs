use std::collections::HashSet;

use super::{omaha_hi_evaluator, EvaluatorError};

use itertools::Itertools;
use phf::phf_map;

use crate::{
    core::{Card, Value},
    poker::ranks::{BasicRank, LowA5Rank, OmahaHiLoRank},
};

/// Evaluates the Omaha hi/lo hand for one player
///
/// Returns a `OmahaHiLoRank`. If the player's hand contains less than 4 cards or the board contains
/// less than 3 cards, then an error will return.
///
/// This implementation does not support the use of duplicate cards. If duplicate cards are found
/// when both the player's cards and the board are chained, a `FailedToCalculateRank` error will
/// return.
pub fn evaluate_hand(
    player_hand: &Vec<Card>,
    board: &Vec<Card>,
) -> Result<OmahaHiLoRank, EvaluatorError> {
    if player_hand.len() < 4 {
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Player hand".to_string(),
            expected_count: 4,
            actual_count: player_hand.len() as u64,
        });
        // Player hand does not have at least 4 cards
    }

    if board.len() < 3 {
        // 3 because it allows for evaluation on flop-only flop-turn-only boards
        return Err(EvaluatorError::NotEnoughCards {
            card_set_type: "Board".to_string(),
            expected_count: 3,
            actual_count: board.len() as u64,
        });
        // Board does not have at least 3 cards
    }

    let hi_hand = omaha_hi_evaluator::evaluate_hand(player_hand, board)?;
    let mut lo_hand: Option<LowA5Rank> = None;

    let player_hand_sub_8: Vec<Card> = player_hand
        .iter()
        .filter(|card| card.value <= Value::Eight || card.value == Value::Ace)
        .cloned()
        .collect();

    let board_sub_8: Vec<Card> = board
        .iter()
        .filter(|card| card.value <= Value::Eight || card.value == Value::Ace)
        .cloned()
        .collect();

    if player_hand_sub_8.len() >= 2 && board_sub_8.len() >= 3 {
        let hand_combinations: Vec<Vec<Card>> =
            player_hand_sub_8.iter().cloned().combinations(2).collect();
        let board_combinations: Vec<Vec<Card>> =
            board_sub_8.iter().cloned().combinations(3).collect();

        lo_hand = hand_combinations
            .iter()
            .cartesian_product(board_combinations.iter())
            .map(|(hand_combo, board_combo)| {
                let cards: HashSet<Card> = hand_combo
                    .iter()
                    .chain(board_combo.iter())
                    .cloned()
                    .collect();
                if cards.len() != 5 {
                    return None;
                }

                let bit_strength = cards
                    .iter()
                    .fold(0, |acc, card| acc | (1 << ((card.value as u8 + 1) % 13)));

                if let Some(&(strength, hand_rank, sub_rank, desc)) = LO_8_MAP.get(&bit_strength) {
                    Some(LowA5Rank(BasicRank {
                        strength,
                        hand_rank,
                        sub_rank,
                        description: Some(desc.to_string()),
                    }))
                } else {
                    None
                }
            })
            .fold(None, |acc, rank| if acc < rank { rank } else { acc });
    }

    Ok(OmahaHiLoRank {
        hi_rank: hi_hand,
        lo_rank: lo_hand,
    })
}

static LO_8_MAP: phf::Map<u8, (u32, u16, u16, &'static str)> = phf_map! {
    0xf8u8 => (1, 1, 1, "8-7-6-5-4"),
    0xf4u8 => (2, 1, 2, "8-7-6-5-3"),
    0xf2u8 => (3, 1, 3, "8-7-6-5-2"),
    0xf1u8 => (4, 1, 4, "8-7-6-5-A"),
    0xecu8 => (5, 1, 5, "8-7-6-4-3"),
    0xeau8 => (6, 1, 6, "8-7-6-4-2"),
    0xe9u8 => (7, 1, 7, "8-7-6-4-A"),
    0xe6u8 => (8, 1, 8, "8-7-6-3-2"),
    0xe5u8 => (9, 1, 9, "8-7-6-3-A"),
    0xe3u8 => (10, 1, 10, "8-7-6-2-A"),
    0xdcu8 => (11, 1, 11, "8-7-5-4-3"),
    0xdau8 => (12, 1, 12, "8-7-5-4-2"),
    0xd9u8 => (13, 1, 13, "8-7-5-4-A"),
    0xd6u8 => (14, 1, 14, "8-7-5-3-2"),
    0xd5u8 => (15, 1, 15, "8-7-5-3-A"),
    0xd3u8 => (16, 1, 16, "8-7-5-2-A"),
    0xceu8 => (17, 1, 17, "8-7-4-3-2"),
    0xcdu8 => (18, 1, 18, "8-7-4-3-A"),
    0xcbu8 => (19, 1, 19, "8-7-4-2-A"),
    0xc7u8 => (20, 1, 20, "8-7-3-2-A"),
    0xbcu8 => (21, 1, 21, "8-6-5-4-3"),
    0xbau8 => (22, 1, 22, "8-6-5-4-2"),
    0xb9u8 => (23, 1, 23, "8-6-5-4-A"),
    0xb6u8 => (24, 1, 24, "8-6-5-3-2"),
    0xb5u8 => (25, 1, 25, "8-6-5-3-A"),
    0xb3u8 => (26, 1, 26, "8-6-5-2-A"),
    0xaeu8 => (27, 1, 27, "8-6-4-3-2"),
    0xadu8 => (28, 1, 28, "8-6-4-3-A"),
    0xabu8 => (29, 1, 29, "8-6-4-2-A"),
    0xa7u8 => (30, 1, 30, "8-6-3-2-A"),
    0x9eu8 => (31, 1, 31, "8-5-4-3-2"),
    0x9du8 => (32, 1, 32, "8-5-4-3-A"),
    0x9bu8 => (33, 1, 33, "8-5-4-2-A"),
    0x97u8 => (34, 1, 34, "8-5-3-2-A"),
    0x8fu8 => (35, 1, 35, "8-4-3-2-A"),
    0x7cu8 => (36, 2, 1, "7-6-5-4-3"),
    0x7au8 => (37, 2, 2, "7-6-5-4-2"),
    0x79u8 => (38, 2, 3, "7-6-5-4-A"),
    0x76u8 => (39, 2, 4, "7-6-5-3-2"),
    0x75u8 => (40, 2, 5, "7-6-5-3-A"),
    0x73u8 => (41, 2, 6, "7-6-5-2-A"),
    0x6eu8 => (42, 2, 7, "7-6-4-3-2"),
    0x6du8 => (43, 2, 8, "7-6-4-3-A"),
    0x6bu8 => (44, 2, 9, "7-6-4-2-A"),
    0x67u8 => (45, 2, 10, "7-6-3-2-A"),
    0x5eu8 => (46, 2, 11, "7-5-4-3-2"),
    0x5du8 => (47, 2, 12, "7-5-4-3-A"),
    0x5bu8 => (48, 2, 13, "7-5-4-2-A"),
    0x57u8 => (49, 2, 14, "7-5-3-2-A"),
    0x4fu8 => (50, 2, 15, "7-4-3-2-A"),
    0x3eu8 => (51, 3, 1, "6-5-4-3-2"),
    0x3du8 => (52, 3, 2, "6-5-4-3-A"),
    0x3bu8 => (53, 3, 3, "6-5-4-2-A"),
    0x37u8 => (54, 3, 4, "6-5-3-2-A"),
    0x2fu8 => (55, 3, 5, "6-4-3-2-A"),
    0x1fu8 => (56, 4, 1, "5-4-3-2-A"),
};
