//! This module contains the implementation of HighRank.

use crate::core::Value;
use std::cmp::Ordering;

/// Distinguhes a hand rank relative to finding the best high hand.
/// 
/// This struct is typically returned by evaluators that evaluate a high hand component.
#[derive(Copy, Clone, Debug)]
pub struct HighRank {
    rank_strength: u16,
    hand_rank: u8,
    sub_rank: u16,
}

impl HighRank {
    /// Creates a new HighRank struct
    pub fn new(strength: u16) -> Self {
        let mut hand_rank: u8 = 0;
        let mut sub_rank: u16 = 0;

        if strength >= 1 {
            let mut ranks_left = strength - 1;

            // distinct combos from high card to straight flush
            let strength_threshold = [1277, 2860, 858, 858, 10, 1277, 156, 156, 10];

            for (i, &subranks) in strength_threshold.iter().enumerate().rev() {
                if ranks_left < subranks {
                    hand_rank = (i + 1) as u8;
                    sub_rank = subranks - ranks_left;
                    break;
                }
                ranks_left -= subranks;
            }
        }

        Self {
            rank_strength: 7463 - strength,
            hand_rank: hand_rank,
            sub_rank: sub_rank,
        }
    }

    /// Gets the hand rank's strength.
    pub fn get_rank_strength(&self) -> u16 {
        self.rank_strength
    }

    pub fn get_hand_rank(&self) -> u8 {
        self.hand_rank
    }

    pub fn get_sub_rank(&self) -> u16 {
        self.sub_rank
    }

    /// Returns the string for the associated hand.
    ///
    /// The string is user-interperable string of the hand strength and can be used for displaying
    /// to the user.
    pub fn get_string(&self) -> Result<String, &'static str> {
        let hand_category;
        match self.hand_rank {
            1 => {
                hand_category = "High";

                if self.sub_rank < 1 || self.sub_rank > 1277 {
                    return Err("Sub rank for high card was not valid");
                }

                let sub_str: &str;
                if self.sub_rank > 0 && self.sub_rank <= 4 {
                    sub_str = "7";
                } else if self.sub_rank > 4 && self.sub_rank <= 18 {
                    sub_str = "8";
                } else if self.sub_rank > 18 && self.sub_rank <= 52 {
                    sub_str = "9";
                } else if self.sub_rank > 52 && self.sub_rank <= 121 {
                    sub_str = "10";
                } else if self.sub_rank > 121 && self.sub_rank <= 246 {
                    sub_str = "Jack";
                } else if self.sub_rank > 246 && self.sub_rank <= 455 {
                    sub_str = "Queen";
                } else if self.sub_rank > 455 && self.sub_rank <= 784 {
                    sub_str = "King";
                } else if self.sub_rank > 784 && self.sub_rank <= 1277 {
                    sub_str = "Ace";
                } else {
                    return Err("Sub rank for high card was not valid");
                }

                return Ok(Vec::from([sub_str.to_owned(), hand_category.to_owned()]).join(" "));
            },
            2 => {
                hand_category = "Pair";

                let sub_str;
                match Value::from_int((self.sub_rank - 1) / 220) {
                    Some(val) => {
                        sub_str = val.get_readable_string() + "s";
                    }
                    None => {
                        return Err("Sub rank for one pair was not valid");
                    }
                }

                return Ok(Vec::from([hand_category.to_owned(), "of".to_owned(), sub_str.to_owned()]).join(" "));
            },
            3 => {
                hand_category = "Two Pair";

                let first_pair_rank = (((2*(self.sub_rank - 1) / 11) as f64 + 0.25).sqrt()-0.5).floor() as u16 + 1;
                let sec_pair_kick_rank = self.sub_rank - (first_pair_rank - 1) * first_pair_rank / 2 * 11;

                let sub_str;
                match (Value::from_int(first_pair_rank), Value::from_int((sec_pair_kick_rank - 1) / 11)) {
                    (Some(first_pair), Some(sec_pair)) => {
                        sub_str = Vec::from([first_pair.get_readable_string() + "s", "and".to_string(), sec_pair.get_readable_string() + "s"]).join(" ");
                    },
                    _ => {
                        return Err("Sub rank for two pair was not valid");
                    }
                }
                
                return Ok(Vec::from([hand_category.to_owned(), "of".to_string(), sub_str]).join(" "));
            },
            4 => {
                hand_category = "Trip";

                let sub_str;
                match Value::from_int((self.sub_rank - 1) / 66) {
                    Some(val) => {
                        sub_str = val.get_readable_string() + "s";
                    }
                    None => {
                        return Err("Sub rank for three of a kind was not valid");
                    }
                }

                return Ok(Vec::from([hand_category.to_owned(), sub_str.to_owned()]).join(" "));
            },
            5 => {
                hand_category = "Straight";

                if self.sub_rank < 1 || self.sub_rank > 10 {
                    return Err("Sub rank for straight was not valid");
                }

                let sub_str = Value::from_int(self.sub_rank + 2).unwrap().get_readable_string();
                
                return Ok(Vec::from([sub_str.to_owned(), "High".to_string(), hand_category.to_owned()]).join(" "));
            },
            6 => {
                hand_category = "Flush";

                let sub_str: &str;
                if self.sub_rank > 0 && self.sub_rank <= 4 {
                    sub_str = "7";
                } else if self.sub_rank > 4 && self.sub_rank <= 18 {
                    sub_str = "8";
                } else if self.sub_rank > 18 && self.sub_rank <= 52 {
                    sub_str = "9";
                } else if self.sub_rank > 52 && self.sub_rank <= 121 {
                    sub_str = "10";
                } else if self.sub_rank > 121 && self.sub_rank <= 246 {
                    sub_str = "Jack";
                } else if self.sub_rank > 246 && self.sub_rank <= 455 {
                    sub_str = "Queen";
                } else if self.sub_rank > 455 && self.sub_rank <= 784 {
                    sub_str = "King";
                } else if self.sub_rank > 784 && self.sub_rank <= 1277 {
                    sub_str = "Ace";
                } else {
                    return Err("Sub rank for flush was not valid");
                }

                return Ok(Vec::from([sub_str.to_owned(), "High".to_string(), hand_category.to_owned()]).join(" "));
            },
            7 => {
                // Full house

                let trip_rank = (self.sub_rank - 1) / 12;
                let mut pair_rank = (self.sub_rank - 1) % 12;

                if pair_rank >= trip_rank {
                    pair_rank += 1;
                }

                match (Value::from_int(trip_rank), Value::from_int(pair_rank)) {
                    (Some(trip_val), Some(pair_val)) => {
                        return Ok(Vec::from([trip_val.get_readable_string() + "s", "Full of".to_string(), pair_val.get_readable_string() + "s"]).join(" "))
                    },
                    _ => {
                        return Err("Sub rank for full house was not valid");
                    }
                }
            },
            8 => {
                hand_category = "Quad";

                let sub_str;
                match Value::from_int((self.sub_rank - 1) / 12) {
                    Some(val) => {
                        sub_str = val.get_readable_string() + "s";
                    }
                    None => {
                        return Err("Sub rank for four of a kind was not valid");
                    }
                }

                return Ok(Vec::from([hand_category.to_owned(), sub_str.to_owned()]).join(" "));
            },
            9 => {
                hand_category = "Straight Flush";

                if self.sub_rank < 1 || self.sub_rank > 10 {
                    return Err("Sub rank for straight was not valid");
                }

                let sub_str = Value::from_int(self.sub_rank + 2).unwrap().get_readable_string();
                
                return Ok(Vec::from([sub_str.to_owned(), "High".to_string(), hand_category.to_owned()]).join(" "));
            },
            _ => {
                return Err("Hand rank did not have a valid hand category");
            }
        }
    }
}

// PartialOrd and PartialEq unfortunately are repeated for all Rank types
// This is because there is no way to implement generic types for foriegn traits, so alas
impl PartialOrd for HighRank {
    fn partial_cmp(&self, other: &HighRank) -> Option<Ordering> {
        Some(self.get_rank_strength().cmp(&other.get_rank_strength()))
    }
}

impl PartialEq for HighRank {
    fn eq(&self, other: &HighRank) -> bool {
        self.get_rank_strength() == other.get_rank_strength()
    }

    fn ne(&self, other: &HighRank) -> bool {
        self.get_rank_strength() != other.get_rank_strength()
    }
}
