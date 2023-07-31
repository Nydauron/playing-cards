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
/// let rank = &high_evaluator::evaluate_hand(&hand, &board).unwrap();
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
/// let rank = &high_evaluator::evaluate_hand(&hand, &board).unwrap();
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
/// let hero_rank = &high_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = &high_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
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
/// let rank = &low_evaluator::evaluate_hand(&hand, &Vec::new()).unwrap();
///
/// assert_eq!(rank.description.as_ref().unwrap(), "Ace High");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_evaluator};
///
/// let hand = Card::vec_from_str("2c4dKs2dKd").unwrap();
///
/// let rank = &low_evaluator::evaluate_hand(&hand, &Vec::new()).unwrap();
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
/// let hero_rank = &low_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = &low_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "10 High");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "6 High Straight");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villan's
/// ```
pub mod low_evaluator;

/// An evaluator for Omaha High hands.
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
/// let rank = &omaha_hi_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// // Notice: Even though we have Aces in our hand, we can only use 2 cards from out hand to
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
/// let rank = &omaha_hi_evaluator::evaluate_hand(&hand, &board).unwrap();
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
/// let hero_rank = &omaha_hi_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = &omaha_hi_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "10 High Straight");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "Trip Aces");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villans's
/// ```
pub mod omaha_hi_evaluator;

/// An evaluator for Omaha Hi/Lo hands.
///
/// Similar to the Omaha Hi evalautor, this requires 4 player cards and at least 3 cards from the
/// board in order to evaluate properly. Only 2 cards may be used from the player's hand to fulfill
/// their high hand and lo hand (can be the same pair or different). The lo hand only plays if the
/// player cards has at least 2 cards and the board has at least 3 cards with a distinct rank
/// between A-8. Since the lo hand only applies under a condition, the return value of the evaluator
/// contains Option types to signify the player does not have a lo hand.
///
/// Examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator};
///
/// let hand = Card::vec_from_str("2cAsAcKc").unwrap();
/// let board = Card::vec_from_str("Ks2sTd8h7d").unwrap();
///
/// let ranks = &omaha_hilo_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// // Notice: Even though we have Aces in our hand, we can only use 2 cards from out hand to
/// // make the best hand (e.g. the K and the 2 pair with the board).
/// assert_eq!(ranks[0].as_ref().unwrap().description.as_ref().unwrap(), "Two Pair of Kings and 2s");
/// // While we have A2 in our hand, the board has 278, which is only 4 distinct ranks, so no
/// // lo rank exists
/// assert_eq!(ranks[1], None);
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator};
///
/// let hand = Card::vec_from_str("As2d5sAd").unwrap();
/// let board = Card::vec_from_str("Tc5c3s6c8c").unwrap();
///
/// let ranks = &omaha_hilo_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// assert_eq!(ranks[0].as_ref().unwrap().description.as_ref().unwrap(), "Pair of Aces");
/// assert_eq!(ranks[1].as_ref().unwrap().description.as_ref().unwrap(), "6-5-3-2-A");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator};
///
/// let hero_hand = Card::vec_from_str("5s6c9s7c").unwrap();
/// let villan_hand = Card::vec_from_str("AhKdAsTh").unwrap();
/// let board = Card::vec_from_str("8hTcAdQs6s").unwrap();
///
/// let hero_ranks = &omaha_hilo_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_ranks = &omaha_hilo_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// assert_eq!(hero_ranks[0].as_ref().unwrap().description.as_ref().unwrap(), "10 High Straight");
/// assert_eq!(villan_ranks[0].as_ref().unwrap().description.as_ref().unwrap(), "Trip Aces");
///
/// assert_eq!(hero_ranks[1].as_ref().unwrap().description.as_ref().unwrap(), "8-7-6-5-A");
/// assert_eq!(villan_ranks[1], None);
///
/// println!("{:?}", hero_ranks[1]);
/// println!("{:?}", villan_ranks[1]);
/// assert!(hero_ranks[0] > villan_ranks[0]); // Hero's hi hand is better than the villans's
/// assert!(hero_ranks[1].gt(&villan_ranks[1])); // Hero's lo hand is better than the villans's
/// ```
pub mod omaha_hilo_evaluator;

/// An evaluator for Drawmaha hands.
///
/// Drawmaha is a combination of Five Card Draw and Big O (an Omaha varient). This evaluator makes
/// use of both the HighEvaluator and OmahaHighEvaluator.
///
/// Examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::drawmaha_evaluator};
///
/// let hand = Card::vec_from_str("5cAsKdKcAc").unwrap();
/// let board = Card::vec_from_str("Ks6s2d8c3h").unwrap();
///
/// let rank = drawmaha_evaluator::evaluate_hand(&hand, &board).unwrap();
/// let omaha_rank = &rank[0];
/// let draw_rank = &rank[1];
///
/// assert_eq!(omaha_rank.description.as_ref().unwrap(), "Trip Kings");
/// assert_eq!(draw_rank.description.as_ref().unwrap(), "Two Pair of Aces and Kings");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::drawmaha_evaluator};
///
/// let hand = Card::vec_from_str("3s9sAsTsQs").unwrap();
/// let board = Card::vec_from_str("4d9hQdTcKh").unwrap();
///
/// let rank = drawmaha_evaluator::evaluate_hand(&hand, &board).unwrap();
/// let omaha_rank = &rank[0];
/// let draw_rank = &rank[1];
///
/// assert_eq!(omaha_rank.description.as_ref().unwrap(), "Two Pair of Queens and 10s");
/// assert_eq!(draw_rank.description.as_ref().unwrap(), "Ace High Flush");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::drawmaha_evaluator};
///
/// let hero_hand = Card::vec_from_str("Tc9sJs8hQd").unwrap();
/// let villan_hand = Card::vec_from_str("AsQcKdQhAc").unwrap();
/// let board = Card::vec_from_str("8d8s3cAh7d").unwrap();
///
/// let hero_rank = drawmaha_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = drawmaha_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// // Omaha Rank
/// assert_eq!(hero_rank[0].description.as_ref().unwrap(), "Trip 8s");
/// assert_eq!(villan_rank[0].description.as_ref().unwrap(), "Aces Full of 8s");
///
/// assert!(hero_rank[0] < villan_rank[0]); // Villan's hand is better than the hero's
///
/// // 5-card Draw Rank
/// assert_eq!(hero_rank[1].description.as_ref().unwrap(), "Queen High Straight");
/// assert_eq!(villan_rank[1].description.as_ref().unwrap(), "Two Pair of Aces and Queens");
///
/// assert!(hero_rank[1] > villan_rank[1]); // Hero's hand is better than the villan's
/// ```
pub mod drawmaha_evaluator;
