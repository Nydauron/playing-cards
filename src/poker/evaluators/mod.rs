//! Contains all hand evaluators.

pub mod evaluator;

mod evaluator_errors;
pub use self::evaluator_errors::EvaluatorError;

/// The namespace for the high evaluator.
///
/// This evaluator is typically used for games like Texas Hold'em, Five Card Draw, and Stud.
///
/// Examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator};
///
/// let hand = Card::vec_from_str("5h5s").unwrap();
/// let board = Card::vec_from_str("2dTdKs5sAc").unwrap();
///
/// let rank = &high_evaluator::evaluate_hand(&hand, &board).unwrap()[0];
///
/// assert_eq!(rank.get_description().unwrap(), "Trip 5s");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator};
///
/// let hand = Card::vec_from_str("KhAs").unwrap();
/// let board = Card::vec_from_str("2cQdKs5dAd").unwrap();
///
/// let rank = &high_evaluator::evaluate_hand(&hand, &board).unwrap()[0];
///
/// assert_eq!(rank.get_description().unwrap(), "Two Pair of Aces and Kings");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator};
///
/// let hero_hand = Card::vec_from_str("KhQc").unwrap();
/// let villan_hand = Card::vec_from_str("Ac2c").unwrap();
/// let board = Card::vec_from_str("AhKsQs9c2h").unwrap();
///
/// let hero_rank = &high_evaluator::evaluate_hand(&hero_hand, &board).unwrap()[0];
/// let villan_rank = &high_evaluator::evaluate_hand(&villan_hand, &board).unwrap()[0];
///
/// assert_eq!(hero_rank.get_description().unwrap(), "Two Pair of Kings and Queens");
/// assert_eq!(villan_rank.get_description().unwrap(), "Two Pair of Aces and 2s");
///
/// assert!(hero_rank < villan_rank); // Villan's hand is better than the hero's
/// ```
pub mod high_evaluator;

/// The namespace for the low evaluator.
///
/// This evaluator is typically used for games like 2-7 Lowball Draw.
///
/// Examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_evaluator};
///
/// let hand = Card::vec_from_str("2dTd3s5sAc").unwrap();
///
/// let rank = &low_evaluator::evaluate_hand(&hand, &Vec::new()).unwrap()[0];
///
/// assert_eq!(rank.get_description().unwrap(), "Ace High");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_evaluator};
///
/// let hand = Card::vec_from_str("2c4dKs2dKd").unwrap();
///
/// let rank = &low_evaluator::evaluate_hand(&hand, &Vec::new()).unwrap()[0];
///
/// assert_eq!(rank.get_description().unwrap(), "Two Pair of Kings and 2s");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_evaluator};
///
/// let hero_hand = Card::vec_from_str("6h7h2s3cTd").unwrap();
/// let villan_hand = Card::vec_from_str("2c3s4s5s6d").unwrap();
/// let board = vec![];
///
/// let hero_rank = &low_evaluator::evaluate_hand(&hero_hand, &board).unwrap()[0];
/// let villan_rank = &low_evaluator::evaluate_hand(&villan_hand, &board).unwrap()[0];
///
/// assert_eq!(hero_rank.get_description().unwrap(), "10 High");
/// assert_eq!(villan_rank.get_description().unwrap(), "6 High Straight");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villan's
/// ```
pub mod low_evaluator;

// mod omaha_hi_evaluator;
// pub use self::omaha_hi_evaluator::OmahaHighEvaluator;

// mod drawmaha_evaluator;
// pub use self::drawmaha_evaluator::DrawmahaEvaluator;
