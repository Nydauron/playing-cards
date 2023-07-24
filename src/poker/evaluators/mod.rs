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
/// assert_eq!(rank.description.as_ref().unwrap(), "Trip 5s");
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
/// assert_eq!(rank.description.as_ref().unwrap(), "Two Pair of Aces and Kings");
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
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "Two Pair of Kings and Queens");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "Two Pair of Aces and 2s");
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
/// assert_eq!(rank.description.as_ref().unwrap(), "Ace High");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_evaluator};
///
/// let hand = Card::vec_from_str("2c4dKs2dKd").unwrap();
///
/// let rank = &low_evaluator::evaluate_hand(&hand, &Vec::new()).unwrap()[0];
///
/// assert_eq!(rank.description.as_ref().unwrap(), "Two Pair of Kings and 2s");
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
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "10 High");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "6 High Straight");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villan's
/// ```
pub mod low_evaluator;

/// An evaluator for Omaha high hands.
///
/// The evaluator requires that the player has at least 4 cards and the board has at least 3
/// cards. In Omaha and Omaha-varients, the player is required to use only 2 cards from their
/// hand and 3 from the board. This evaluator permutates through these combinations in parallel
/// with the help of map-reduce.
///
/// Some games that can make use of this evaluator include but are not limited to Omaha, Omaha 8
/// (Hi/Lo), Big O, and Drawmaha.
///
/// Examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator};
///
/// let hand = Card::vec_from_str("2cAsAcKc").unwrap();
/// let board = Card::vec_from_str("Ks2sTd8h7d").unwrap();
///
/// let rank = &omaha_hi_evaluator::evaluate_hand(&hand, &board).unwrap()[0];
///
/// // Notice: Even though we can Aces in our hand, we can only use 2 cards from out hand to
/// // make the best hand (e.g. the K and the 2 pair with the board).
/// assert_eq!(rank.description.as_ref().unwrap(), "Two Pair of Kings and 2s");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator};
///
/// let hand = Card::vec_from_str("AcKhKsTd").unwrap();
/// let board = Card::vec_from_str("Tc5c3s6cQc").unwrap();
///
/// let rank = &omaha_hi_evaluator::evaluate_hand(&hand, &board).unwrap()[0];
///
/// // Notice: Even though we have the Ace of Clubs in out hand, we do not have a flush, as we
/// // need another club within our hand.
/// assert_eq!(rank.description.as_ref().unwrap(), "Pair of Kings");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator};
///
/// let hero_hand = Card::vec_from_str("5s6c9s7c").unwrap();
/// let villan_hand = Card::vec_from_str("AhKdAsTh").unwrap();
/// let board = Card::vec_from_str("8hTcAdQs6s").unwrap();
///
/// let hero_rank = &omaha_hi_evaluator::evaluate_hand(&hero_hand, &board).unwrap()[0];
/// let villan_rank = &omaha_hi_evaluator::evaluate_hand(&villan_hand, &board).unwrap()[0];
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "10 High Straight");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "Trip Aces");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villans's
/// ```
pub mod omaha_hi_evaluator;

// mod drawmaha_evaluator;
// pub use self::drawmaha_evaluator::DrawmahaEvaluator;
