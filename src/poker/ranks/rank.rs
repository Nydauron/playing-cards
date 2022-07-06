//! This module contains all Rank types and traits.

use std::fmt::Debug;

/// A trait for comparing Ranks.
///
/// Structs that implement Rank can only be compared with themselves with the help of
/// `is_stronger()`, `is_weaker()`, and `is_equal()` to determine relative hand
/// strength.
pub trait Rank : PartialOrd + Debug {
    /// Gets the hand rank's strength.
    fn get_rank_strength(&self) -> u64;

    /// Returns the string for the associated hand.
    /// 
    /// The string is user-interperable string of the hand strength and can be used for displaying
    /// to the user.
    fn get_string(&self) -> Result<String, &'static str>;

}
