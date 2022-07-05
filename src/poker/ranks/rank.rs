//! This module contains all Rank types and traits.

use std::cmp::Ordering;
use std::any::Any;
use std::fmt::Debug;

/// A trait for comparing Ranks.
///
/// Structs that implement Rank can only be compared with themselves with the help of
/// `is_stronger()`, `is_weaker()`, and `is_equal()` to determine relative hand
/// strength.
pub trait Rank : Debug {
    /// Gets the hand rank's strength.
    fn get_rank_strength(&self) -> u64;

    /// Returns the string for the associated hand.
    /// 
    /// The string is user-interperable string of the hand strength and can be used for displaying
    /// to the user.
    fn get_string(&self) -> Result<String, &'static str>;

}

impl PartialOrd<Box<dyn Rank>> for Box<dyn Rank> {
    fn partial_cmp(&self, other: &Box<dyn Rank>) -> Option<Ordering> {
        if self.type_id() == other.type_id() {
            Some(self.get_rank_strength().cmp(&other.get_rank_strength()))
        } else {
            None
        }
    }
}

impl PartialEq<Box<dyn Rank>> for Box<dyn Rank> {
    fn eq(&self, other: &Box<dyn Rank>) -> bool {
        if self.type_id() == other.type_id() {
            self.get_rank_strength() == other.get_rank_strength()
        } else {
            false
        }
    }

    fn ne(&self, other: &Box<dyn Rank>) -> bool {
        if self.type_id() == other.type_id() {
            self.get_rank_strength() != other.get_rank_strength()
        } else {
            true
        }
    }
}
