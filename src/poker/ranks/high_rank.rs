//! This module contains the implementation of HighRank.

use super::Rank;
use crate::core::Value;

/// Distinguhes a hand rank relative to finding the best high hand.
/// 
/// This struct is typically returned by evaluators that evaluate a high hand component.
#[derive(Copy, Clone, Debug)]
pub struct HighRank {
    rank_strength: u64,
}

impl HighRank {
    /// Creates a new HighRank struct
    pub fn new(strength: u64) -> Self {
        Self { rank_strength: strength }
    }
}

impl Rank for HighRank {
    fn get_rank_strength(&self) -> u64 {
        self.rank_strength    
    }

    fn get_string(&self) -> Result<String, &'static str> {
        let hand_rank = self.rank_strength;
        let hand_category;
        let sub_rank = hand_rank & 0xFFF;
        match hand_rank >> 12 {
            1 => {
                hand_category = "High";

                if sub_rank < 1 || sub_rank > 1277 {
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

                return Ok(Vec::from([sub_str.to_owned(), hand_category.to_owned()]).join(" "));
            },
            2 => {
                hand_category = "Pair";

                let sub_str;
                match Value::from_int((sub_rank - 1) / 220) {
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

                let first_pair_rank = (((2*(sub_rank - 1) / 11) as f64 + 0.25).sqrt()-0.5).floor() as u64 + 1;
                let sec_pair_kick_rank = sub_rank - (first_pair_rank - 1) * first_pair_rank / 2 * 11;

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
                match Value::from_int((sub_rank - 1) / 66) {
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

                if sub_rank < 1 || sub_rank > 10 {
                    return Err("Sub rank for straight was not valid");
                }

                let sub_str = Value::from_int(sub_rank + 2).unwrap().get_readable_string();
                
                return Ok(Vec::from([sub_str.to_owned(), "High".to_string(), hand_category.to_owned()]).join(" "));
            },
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

                return Ok(Vec::from([sub_str.to_owned(), "High".to_string(), hand_category.to_owned()]).join(" "));
            },
            7 => {
                // Full house

                let trip_rank = (sub_rank - 1) / 12;
                let mut pair_rank = (sub_rank - 1) % 12;

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
                match Value::from_int((sub_rank - 1) / 12) {
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

                if sub_rank < 1 || sub_rank > 10 {
                    return Err("Sub rank for straight was not valid");
                }

                let sub_str = Value::from_int(sub_rank + 2).unwrap().get_readable_string();
                
                return Ok(Vec::from([sub_str.to_owned(), "High".to_string(), hand_category.to_owned()]).join(" "));
            },
            _ => {
                return Err("Hand rank did not have a valid hand category");
            }
        }
    }
}
