//! Contains all Poker hand evaluators
//!
//! Please note that all evaluators will error if the appropriate number of cards are given or if
//! any duplicate cards are detected.
//!
//! <div class="warning">
//! Signatures in all the of the evaluator functions in this module have changed!
//!
//! In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
//! This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
//! <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
//! snippets to fail to compile due to insufficent information on size or type inferencing, hence
//! why this is a breaking change. If you used the argument type for inferencing the caller
//! argument types, then they need to be refactored. Please see the below examples:
//! </div>

mod evaluator_errors;
pub use self::evaluator_errors::EvaluatorError;

/// An evaluator for high hands
///
/// This evaluator is typically used for games like Texas Hold'em, Five Card Draw, and Stud.
///
/// ## Normal examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator};
///
/// let hand = Card::vec_from_str("5h5s").unwrap();
/// let board = Card::vec_from_str("2dTdKs5dAc").unwrap();
///
/// let mut all_cards = hand.clone();
/// all_cards.extend(board.iter());
///
/// let rank = high_evaluator::evaluate_hand(&all_cards).unwrap();
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
/// let mut all_cards = hand.clone();
/// all_cards.extend(board.iter());
///
/// let rank = high_evaluator::evaluate_hand(&all_cards).unwrap();
///
/// assert_eq!(rank.description.as_ref().unwrap(), "Two Pair of Aces and Kings");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator};
///
/// let mut hero_hand = Card::vec_from_str("KhQc").unwrap();
/// let mut villan_hand = Card::vec_from_str("Ac2c").unwrap();
/// let board = Card::vec_from_str("AhKsQs9c2h").unwrap();
///
/// hero_hand.extend(board.iter());
/// villan_hand.extend(board.iter());
///
/// let hero_rank = high_evaluator::evaluate_hand(&hero_hand).unwrap();
/// let villan_rank = high_evaluator::evaluate_hand(&villan_hand).unwrap();
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "Two Pair of Kings and Queens");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "Two Pair of Aces and 2s");
///
/// assert!(hero_rank < villan_rank); // Villan's hand is better than the hero's
/// ```
///
/// ## Breaking v1.0 changes
/// <div class="warning">
/// In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
/// This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
/// <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
/// snippets to fail to compile due to insufficent information on size or type inferencing, hence
/// why this is a breaking change. If you used the argument type for inferencing the caller
/// argument types, then they need to be refactored. Please see the below examples:
/// </div>
///
/// ```compile_fail
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from([Card::from(1), Card::from(2)]);
/// let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);
///
/// evaluate_hand(
///     &player_hand
///         .iter()
///         .chain(board.iter())
///         .cloned()
///         .collect() // In prior versions, `collect()` would be able to inference the type that
///                    // the iterator should collect into into a `Vec<Card>` without any
///                    // additional type hints. Since `[Card]` does not implement the `Sized`
///                    // trait, the iterator cannot collect the cards into a slice.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from([Card::from(1), Card::from(2)]);
/// let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);
///
/// evaluate_hand(
///     &player_hand
///         .iter()
///         .chain(board.iter())
///         .cloned()
///         .collect::<Vec<Card>>() // A simple fix would be to add type hints to `collect()` as
///                                 // such.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::high_evaluator::evaluate_hand};
///
/// let five_card_hand = Vec::from(
///     [
///         Card::from(1),
///         Card::from(2),
///         Card::from(12),
///         Card::from(20),
///         Card::from(43),
///     ]
/// );
///
/// evaluate_hand(
///     &five_card_hand // Passing a previously defined vector works exactly as before since
///                     // `Vec<T>` implements the `AsRef<[T]>` trait.
/// );
/// ```
pub mod high_evaluator;

/// An evaluator for 2-7 lowball hands
///
/// This evaluator is typically used for games like 2-7 Lowball Draw.
///
/// ## Normal examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_27_evaluator};
///
/// let hand = Card::vec_from_str("2dTd3s5sAc").unwrap();
///
/// let rank = low_27_evaluator::evaluate_hand(&hand).unwrap();
///
/// assert_eq!(rank.description.as_ref().unwrap(), "Ace High");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_27_evaluator};
///
/// let hand = Card::vec_from_str("2c4dKs2dKd").unwrap();
///
/// let rank = low_27_evaluator::evaluate_hand(&hand).unwrap();
///
/// assert_eq!(rank.description.as_ref().unwrap(), "Two Pair of Kings and 2s");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::low_27_evaluator};
///
/// let hero_hand = Card::vec_from_str("6h7h2s3cTd").unwrap();
/// let villan_hand = Card::vec_from_str("2c3s4s5s6d").unwrap();
///
/// let hero_rank = low_27_evaluator::evaluate_hand(&hero_hand).unwrap();
/// let villan_rank = low_27_evaluator::evaluate_hand(&villan_hand).unwrap();
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "10 High");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "6 High Straight");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villan's
/// ```
///
/// ## Breaking v1.0 changes
/// <div class="warning">
/// In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
/// This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
/// <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
/// snippets to fail to compile due to insufficent information on size or type inferencing, hence
/// why this is a breaking change. If you used the argument type for inferencing the caller
/// argument types, then they need to be refactored. Please see the below examples:
/// </div>
///
/// ```compile_fail
/// use playing_cards::{core::Card, poker::evaluators::low_27_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from([Card::from(1), Card::from(2)]);
/// let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);
///
/// evaluate_hand(
///     &player_hand
///         .iter()
///         .chain(board.iter())
///         .cloned()
///         .collect() // In prior versions, `collect()` would be able to inference the type that
///                    // the iterator should collect into into a `Vec<Card>` without any
///                    // additional type hints. Since `[Card]` does not implement the `Sized`
///                    // trait, the iterator cannot collect the cards into a slice.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::low_27_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from([Card::from(1), Card::from(2)]);
/// let board = Vec::from([Card::from(7), Card::from(5), Card::from(6), Card::from(52)]);
///
/// evaluate_hand(
///     &player_hand
///         .iter()
///         .chain(board.iter())
///         .cloned()
///         .collect::<Vec<Card>>() // A simple fix would be to add type hints to `collect()` as
///                                 // such.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::low_27_evaluator::evaluate_hand};
///
/// let five_card_hand = Vec::from(
///     [
///         Card::from(1),
///         Card::from(2),
///         Card::from(12),
///         Card::from(20),
///         Card::from(43),
///     ]
/// );
///
/// evaluate_hand(
///     &five_card_hand // Passing a previously defined vector works exactly as before since
///                     // `Vec<T>` implements the `AsRef<[T]>` trait.
/// );
/// ```
pub mod low_27_evaluator;

// pub mod low_a5_evaluator;

/// An evaluator for Omaha High hands
///
/// The evaluator requires that the player has at least 4 cards and the board has at least 3
/// cards. In Omaha and Omaha-variants, the player is required to use only 2 cards from their
/// hand and 3 from the board. This evaluator permutates through these combinations in parallel
/// with the help of map-reduce.
///
/// Some games that can make use of this evaluator include but are not limited to Omaha, Omaha 8
/// (Hi-Lo), Big O, and Dramaha.
///
/// ## Normal examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator};
///
/// let hand = Card::vec_from_str("2cAsAcKc").unwrap();
/// let board = Card::vec_from_str("Ks2sTd8h7d").unwrap();
///
/// let rank = omaha_hi_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// // Notice: Even though we have Aces in our hand, we can only use 2 cards from out hand to
/// // make the best hand (e.g. the king and the deuce pair with the board).
/// assert_eq!(rank.description.as_ref().unwrap(), "Two Pair of Kings and 2s");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator};
///
/// let hand = Card::vec_from_str("AcKhKsTd").unwrap();
/// let board = Card::vec_from_str("Tc5c3s6cQc").unwrap();
///
/// let rank = omaha_hi_evaluator::evaluate_hand(&hand, &board).unwrap();
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
/// let hero_rank = omaha_hi_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = omaha_hi_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "10 High Straight");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "Trip Aces");
///
/// assert!(hero_rank > villan_rank); // Hero's hand is better than the villan's
/// ```
///
/// ## Breaking v1.0 changes
/// <div class="warning">
/// In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
/// This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
/// <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
/// snippets to fail to compile due to insufficent information on size or type inferencing, hence
/// why this is a breaking change. If you used the argument type for inferencing the caller
/// argument types, then they need to be refactored. Please see the below examples:
/// </div>
///
/// ```compile_fail
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator::evaluate_hand};
///
/// evaluate_hand(
///     &([Card::from(1), Card::from(2), Card::from(3), Card::from(4)].into_iter().collect()),
///     &([Card::from(7), Card::from(5), Card::from(6), Card::from(52)].into_iter().collect()),
///     // In prior versions, `collect()` would be able to inference the type that
///     // the iterator should collect into into a `Vec<Card>` without any
///     // additional type hints. Since `[Card]` does not implement the `Sized`
///     // trait, the iterator cannot collect the cards into a slice.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator::evaluate_hand};
///
/// evaluate_hand(
///     &([Card::from(1), Card::from(2), Card::from(3), Card::from(4)].into_iter().collect::<Vec<Card>>()),
///     &([Card::from(7), Card::from(5), Card::from(6), Card::from(52)].into_iter().collect::<Vec<Card>>()),
///     // A simple fix would be to add type hints to `collect()` as such.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::omaha_hi_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from(
///     [
///         Card::from(1),
///         Card::from(2),
///         Card::from(12),
///         Card::from(20),
///     ]
/// );
/// let board = Vec::from([Card::from(24), Card::from(14), Card::from(47)]);
///
/// evaluate_hand(
///     &player_hand,
///     &board,
///     // Passing a previously defined vector works exactly as before since `Vec<T>` implements
///     // the `AsRef<[T]>` trait.
/// );
/// ```
pub mod omaha_hi_evaluator;

/// An evaluator for Omaha Hi-Lo hands
///
/// Similar to the Omaha Hi evaluator, this requires 4 player cards and at least 3 cards from the
/// board in order to evaluate properly. Only 2 cards may be used from the player's hand to fulfill
/// their high hand and lo hand (can be the same pair or different). The lo hand only plays if the
/// player cards has at least 2 cards and the board has at least 3 cards with a distinct rank
/// between A-8. Since the lo hand only applies under a condition, the return value of the evaluator
/// contains Option types to signify the player does not have a lo hand.
///
/// ## Normal examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator};
///
/// let hand = Card::vec_from_str("2cAsAcKc").unwrap();
/// let board = Card::vec_from_str("Ks2sTd8h7d").unwrap();
///
/// let ranks = omaha_hilo_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// // Notice: Even though we have Aces in our hand, we can only use 2 cards from out hand to
/// // make the best hand (e.g. the K and the 2 pair with the board).
/// assert_eq!(ranks.hi_rank.description.as_ref().unwrap(), "Two Pair of Kings and 2s");
/// // While we have A2 in our hand, the board has 278, which is only 4 distinct ranks, so no
/// // lo rank exists
/// assert_eq!(ranks.lo_rank, None);
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator};
///
/// let hand = Card::vec_from_str("As2d5sAd").unwrap();
/// let board = Card::vec_from_str("Tc5c3s6c8c").unwrap();
///
/// let ranks = omaha_hilo_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// assert_eq!(ranks.hi_rank.description.as_ref().unwrap(), "Pair of Aces");
/// assert_eq!(ranks.lo_rank.as_ref().unwrap().description.as_ref().unwrap(), "6-5-3-2-A");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator};
///
/// let hero_hand = Card::vec_from_str("5s6c9s7c").unwrap();
/// let villan_hand = Card::vec_from_str("AhKdAsTh").unwrap();
/// let board = Card::vec_from_str("8hTcAdQs6s").unwrap();
///
/// let hero_ranks = omaha_hilo_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_ranks = omaha_hilo_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// assert_eq!(hero_ranks.hi_rank.description.as_ref().unwrap(), "10 High Straight");
/// assert_eq!(villan_ranks.hi_rank.description.as_ref().unwrap(), "Trip Aces");
///
/// assert_eq!(hero_ranks.lo_rank.as_ref().unwrap().description.as_ref().unwrap(), "8-7-6-5-A");
/// assert_eq!(villan_ranks.lo_rank, None);
///
/// assert!(hero_ranks.hi_rank > villan_ranks.hi_rank); // Hero's hi hand is better than the villan's
/// assert!(hero_ranks.lo_rank.gt(&villan_ranks.lo_rank)); // Hero's lo hand is better than the villan's
/// ```
///
/// ## Breaking v1.0 changes
/// <div class="warning">
/// In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
/// This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
/// <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
/// snippets to fail to compile due to insufficent information on size or type inferencing, hence
/// why this is a breaking change. If you used the argument type for inferencing the caller
/// argument types, then they need to be refactored. Please see the below examples:
/// </div>
///
/// ```compile_fail
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator::evaluate_hand};
///
/// evaluate_hand(
///     &([Card::from(1), Card::from(2), Card::from(3), Card::from(4)].into_iter().collect()),
///     &([Card::from(7), Card::from(5), Card::from(6), Card::from(52)].into_iter().collect()),
///     // In prior versions, `collect()` would be able to inference the type that
///     // the iterator should collect into into a `Vec<Card>` without any
///     // additional type hints. Since `[Card]` does not implement the `Sized`
///     // trait, the iterator cannot collect the cards into a slice.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator::evaluate_hand};
///
/// evaluate_hand(
///     &([Card::from(1), Card::from(2), Card::from(3), Card::from(4)].into_iter().collect::<Vec<Card>>()),
///     &([Card::from(7), Card::from(5), Card::from(6), Card::from(52)].into_iter().collect::<Vec<Card>>()),
///     // A simple fix would be to add type hints to `collect()` as such.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::omaha_hilo_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from(
///     [
///         Card::from(1),
///         Card::from(2),
///         Card::from(12),
///         Card::from(20),
///     ]
/// );
/// let board = Vec::from([Card::from(24), Card::from(14), Card::from(47)]);
///
/// evaluate_hand(
///     &player_hand,
///     &board,
///     // Passing a previously defined vector works exactly as before since `Vec<T>` implements
///     // the `AsRef<[T]>` trait.
/// );
/// ```
pub mod omaha_hilo_evaluator;

/// An evaluator for Dramaha High hands
///
/// Dramaha High is a combination of Five Card Draw and Big O (an Omaha variant). This evaluator
/// makes use of both the HighEvaluator and OmahaHighEvaluator.
///
/// ## Normal examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::dramaha_high_evaluator};
///
/// let hand = Card::vec_from_str("5cAsKdKcAc").unwrap();
/// let board = Card::vec_from_str("Ks6s2d8c3h").unwrap();
///
/// let rank = dramaha_high_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// assert_eq!(rank.omaha_rank.description.as_ref().unwrap(), "Trip Kings");
/// assert_eq!(rank.draw_rank.description.as_ref().unwrap(), "Two Pair of Aces and Kings");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::dramaha_high_evaluator};
///
/// let hand = Card::vec_from_str("3s9sAsTsQs").unwrap();
/// let board = Card::vec_from_str("4d9hQdTcKh").unwrap();
///
/// let rank = dramaha_high_evaluator::evaluate_hand(&hand, &board).unwrap();
///
/// assert_eq!(rank.omaha_rank.description.as_ref().unwrap(), "Two Pair of Queens and 10s");
/// assert_eq!(rank.draw_rank.description.as_ref().unwrap(), "Ace High Flush");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::dramaha_high_evaluator};
///
/// let hero_hand = Card::vec_from_str("Tc9sJs8hQd").unwrap();
/// let villan_hand = Card::vec_from_str("AsQcKdQhAc").unwrap();
/// let board = Card::vec_from_str("8d8s3cAh7d").unwrap();
///
/// let hero_rank = dramaha_high_evaluator::evaluate_hand(&hero_hand, &board).unwrap();
/// let villan_rank = dramaha_high_evaluator::evaluate_hand(&villan_hand, &board).unwrap();
///
/// // Omaha Rank
/// assert_eq!(hero_rank.omaha_rank.description.as_ref().unwrap(), "Trip 8s");
/// assert_eq!(villan_rank.omaha_rank.description.as_ref().unwrap(), "Aces Full of 8s");
///
/// assert!(hero_rank.omaha_rank < villan_rank.omaha_rank); // Villan's hand is better than the hero's
///
/// // 5-card Draw Rank
/// assert_eq!(hero_rank.draw_rank.description.as_ref().unwrap(), "Queen High Straight");
/// assert_eq!(villan_rank.draw_rank.description.as_ref().unwrap(), "Two Pair of Aces and Queens");
///
/// assert!(hero_rank.draw_rank > villan_rank.draw_rank); // Hero's hand is better than the villan's
/// ```
///
/// ## Breaking v1.0 changes
/// <div class="warning">
/// In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
/// This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
/// <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
/// snippets to fail to compile due to insufficent information on size or type inferencing, hence
/// why this is a breaking change. If you used the argument type for inferencing the caller
/// argument types, then they need to be refactored. Please see the below examples:
/// </div>
///
/// ```compile_fail
/// use playing_cards::{core::Card, poker::evaluators::dramaha_high_evaluator::evaluate_hand};
///
/// evaluate_hand(
///     &(
///         [
///             Card::from(1),
///             Card::from(2),
///             Card::from(3),
///             Card::from(4),
///             Card::from(10)
///         ]
///         .into_iter()
///         .collect()
///     ),
///     &([Card::from(7), Card::from(5), Card::from(6), Card::from(52)].into_iter().collect()),
///     // In prior versions, `collect()` would be able to inference the type that
///     // the iterator should collect into into a `Vec<Card>` without any
///     // additional type hints. Since `[Card]` does not implement the `Sized`
///     // trait, the iterator cannot collect the cards into a slice.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::dramaha_high_evaluator::evaluate_hand};
///
/// evaluate_hand(
///     &(
///         [
///             Card::from(1),
///             Card::from(2),
///             Card::from(3),
///             Card::from(4),
///             Card::from(10)
///         ]
///         .into_iter()
///         .collect::<Vec<Card>>()
///     ),
///     &([Card::from(7), Card::from(5), Card::from(6), Card::from(52)].into_iter().collect::<Vec<Card>>()),
///     // A simple fix would be to add type hints to `collect()` as such.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::dramaha_high_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from(
///     [
///         Card::from(1),
///         Card::from(2),
///         Card::from(12),
///         Card::from(20),
///     ]
/// );
/// let board = Vec::from([Card::from(24), Card::from(14), Card::from(47)]);
///
/// evaluate_hand(
///     &player_hand,
///     &board,
///     // Passing a previously defined vector works exactly as before since `Vec<T>` implements
///     // the `AsRef<[T]>` trait.
/// );
/// ```
pub mod dramaha_high_evaluator;

/// An evaluator for Badugi hands
///
/// Badugi is a lowball Poker variant where the goal is get the lowest possible hand. Aces are
/// considered low. Hands can range from 1-4 cards, and all cards within the hand must be distinct
/// in suit and rank. All 4-card hands, Badugis, beat all 3 card hands, all 3-card hands beat all
/// 2-card hands, and all 2-card hands beat all 1-card hands.
///
/// ## Normal examples
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::badugi_evaluator};
///
/// let hand = Card::vec_from_str("As4d7cTh").unwrap();
///
/// let rank = badugi_evaluator::evaluate_hand(&hand).unwrap();
///
/// assert_eq!(rank.description.as_ref().unwrap(), "10-high Badugi");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::badugi_evaluator};
///
/// let hand = Card::vec_from_str("QsKs8sJd").unwrap();
///
/// let rank = badugi_evaluator::evaluate_hand(&hand).unwrap();
///
/// assert_eq!(rank.description.as_ref().unwrap(), "Jack-high 2-card hand");
/// ```
///
/// ```rust
/// use playing_cards::{core::Card, poker::evaluators::badugi_evaluator};
///
/// let hero_hand = Card::vec_from_str("Kh3hJcTd").unwrap();
/// let villan_hand = Card::vec_from_str("4d8dTsJh").unwrap();
///
/// let hero_rank = badugi_evaluator::evaluate_hand(&hero_hand).unwrap();
/// let villan_rank = badugi_evaluator::evaluate_hand(&villan_hand).unwrap();
///
/// assert_eq!(hero_rank.description.as_ref().unwrap(), "Jack-high 3-card hand");
/// assert_eq!(villan_rank.description.as_ref().unwrap(), "Jack-high 3-card hand");
///
/// assert!(hero_rank > villan_rank);
/// ```
///
/// ## Breaking v1.0 changes
/// <div class="warning">
/// In older versions, the list of cards input was strictly a <code>&amp;Vec&lt;Card&gt;</code>.
/// This has been loosened to <code>&amp;[Card]</code> to allow for other list types besides
/// <code>&amp;Vec</code>. While this change may not break existing code, it can cause certain code
/// snippets to fail to compile due to insufficent information on size or type inferencing, hence
/// why this is a breaking change. If you used the argument type for inferencing the caller
/// argument types, then they need to be refactored. Please see the below examples:
/// </div>
///
/// ```compile_fail
/// use playing_cards::{core::Card, poker::evaluators::badugi_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from([Card::from(1), Card::from(2)]);
/// let potential_card = Card::from(42);
///
/// evaluate_hand(
///     &player_hand
///         .iter()
///         .chain([potential_card].iter())
///         .cloned()
///         .collect() // In prior versions, `collect()` would be able to inference the type that
///                    // the iterator should collect into into a `Vec<Card>` without any
///                    // additional type hints. Since `[Card]` does not implement the `Sized`
///                    // trait, the iterator cannot collect the cards into a slice.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::badugi_evaluator::evaluate_hand};
///
/// let player_hand = Vec::from([Card::from(1), Card::from(2)]);
/// let potential_card = Card::from(42);
///
/// evaluate_hand(
///     &player_hand
///         .iter()
///         .chain([potential_card].iter())
///         .cloned()
///         .collect::<Vec<Card>>() // A simple fix would be to add type hints to `collect()` as
///                                 // such.
/// );
/// ```
///
/// ```
/// use playing_cards::{core::Card, poker::evaluators::badugi_evaluator::evaluate_hand};
///
/// let four_card_hand = Vec::from(
///     [
///         Card::from(1),
///         Card::from(2),
///         Card::from(12),
///         Card::from(43),
///     ]
/// );
///
/// evaluate_hand(
///     &four_card_hand // Passing a previously defined vector works exactly as before since
///                     // `Vec<T>` implements the `AsRef<[T]>` trait.
/// );
/// ```
pub mod badugi_evaluator;
