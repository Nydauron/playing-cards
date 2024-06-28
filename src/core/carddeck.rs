use std::collections::HashSet;

extern crate rand;

use rand::seq::SliceRandom;
use rand_core::RngCore;

use strum::IntoEnumIterator;

use super::{Card, Suit, Value};

/// A deck of playing cards
///
/// This deck can contain the 52 distinct cards or a custom deck consisting of any assortment of
/// the 52 playing cards upon initialization. Shuffling the `CardDeck` only requires the caller to
/// provide a PRNG, whose type implements the `RngCore` trait.
///
/// <div class="warning">
/// Unlike in previous versions, randomness is now handled by the caller rather than the <code>
/// CardDeck</code> and a seed is no longer passed to use the <code>CardDeck</code>.
/// </div>
///
/// Examples
/// ```rust
/// use playing_cards::core::CardDeck;
///
/// for _ in 0..10 {
///     let mut deck: CardDeck = Default::default();
///
///     // Since we did not shuffle the deck of cards, we should see cards in descending order.
///     for (i, card) in (52..0).zip(deck) {
///         assert_eq!(i, Into::<i32>::into(card));
///     }
/// }
/// ```
///
/// ```rust
/// use playing_cards::core::CardDeck;
/// use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
///
/// let mut deck: CardDeck = Default::default();
/// deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());
///
/// let hand = deck.deal_cards(2, false);
///
/// println!("{:?}", hand.unwrap()); // Two random cards from the deck
/// ```
#[derive(Debug, Clone)]
pub struct CardDeck {
    deck: Vec<Card>,
    muck: Vec<Card>,
}

impl Default for CardDeck {
    fn default() -> Self {
        Self::create_unshuffled_deck()
    }
}

impl CardDeck {
    /// Creates a new shuffled or unshuffled CardDeck
    ///
    /// This constrcutor is a way to create deterministic deck for random but predictable
    /// outcomes. The function will shuffle the deck if a PRNG `rng` is provided. If `rng` is
    /// `None`, then shuffling does not occur, and card order will be of the following order: the
    /// deuce of hearts to the ace of hearts, the deuce of clubs to the ace of clubs, the deuce
    /// of diamonds to the ace of clubs, the deuce of spades to the ace of spades.
    ///
    /// If no PRNG is provided, this is identical to the behavior of `Default::default()`.
    ///
    /// Examples
    /// ```rust
    /// use playing_cards::core::CardDeck;
    /// use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
    ///
    /// for _ in 0..10 {
    ///     let mut seed_bytes = Vec::from(1337_u32.to_ne_bytes());
    ///     seed_bytes.extend_from_slice(&[0u8; 28]);
    ///     let mut deck = CardDeck::new(Some(&mut Xoshiro256PlusPlus::from_seed(seed_bytes.as_slice().try_into().unwrap())));
    ///
    ///     // Every single line should produce the same 5 cards in the same exact order because
    ///     // we gave each deck the same seed.
    ///     let hand = deck.deal_cards(5, false);
    ///     println!("{:?}", hand.unwrap());
    /// }
    /// ```
    ///
    /// ```rust
    /// use playing_cards::core::CardDeck;
    /// use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
    ///
    /// for i in 0..10 {
    ///     let mut seed_bytes = Vec::from((i as u32).to_ne_bytes());
    ///     seed_bytes.extend_from_slice(&[0u8; 28]);
    ///     let mut deck = CardDeck::new(Some(&mut Xoshiro256PlusPlus::from_seed(seed_bytes.as_slice().try_into().unwrap())));
    ///
    ///     // Each line should be different from one another, but if you rerun this code again,
    ///     // it will print out the exact 10 lines again.
    ///     let hand = deck.deal_cards(5, false);
    ///     println!("{:?}", hand.unwrap());
    /// }
    /// ```
    ///
    /// It should be noted that if you intend to use a `SeedableRng + RngCore` type for the PRNG,
    /// the cards within the deck can be predicted if the seed generation is predictable (e.g.
    /// incrementing the seed by one, using UNIX time). It is better to use `new()` in these cases
    /// since the entropy from the system cannot be replicated across systems easily unless the
    /// seed generated is shared.
    pub fn new(rng: Option<&mut dyn RngCore>) -> Self {
        let mut deck = Self::create_unshuffled_deck();

        if let Some(rng) = rng {
            deck.shuffle(rng);
        }

        deck
    }

    /// Creates a new CardDeck with provided `cards`
    ///
    /// Useful if a standard 52-card deck does not fulfill your needs.
    ///
    /// The function will shuffle the deck if a PRNG `rng` is provided. If `rng` is `None`, then
    /// shuffling does not occur, and card order is determined by the order of `cards`.
    pub fn new_custom_deck(cards: Vec<Card>, rng: Option<&mut dyn RngCore>) -> Self {
        let mut deck = Self {
            deck: cards,
            muck: Vec::new(),
        };

        if let Some(rng) = rng {
            deck.shuffle(rng);
        }

        deck
    }

    fn create_unshuffled_deck() -> Self {
        let mut d = Vec::with_capacity(52);

        for s in Suit::iter() {
            for v in Value::iter() {
                d.push(Card { value: v, suit: s });
            }
        }

        CardDeck {
            deck: d,
            muck: Vec::with_capacity(13), // Vec capacity will double if needed, but it minimizes
                                          // the amount of space needed (vector will expand to 52,
                                          // through amortized constant capacity expansion
                                          // (2*current_capacity) rather than starting at 1 and
                                          // exapnding to 64)
        }
    }

    /// Shuffles the deck
    ///
    /// The `rng` argument is a PRNG whose type implements the `RngCore` trait. The use can decide
    /// whether the passed PRNG value is to be seeded from a set value or from entropy.
    pub fn shuffle(&mut self, rng: &mut dyn RngCore) {
        Self::shuffle_cards(&mut self.deck, rng);
    }

    fn shuffle_cards(cards: &mut Vec<Card>, rng: &mut dyn RngCore) {
        cards.shuffle(rng);
    }

    /// Searches the deck and removes cards within provided set of cards
    ///
    /// Returns back a vector of cards that were removed from the deck. Duplicates can be present
    /// in the returned vector if duplicates existed in the deck.
    pub fn strip_cards(&mut self, cards_to_remove: &HashSet<Card>) -> Vec<Card> {
        let removed_cards = self
            .deck
            .iter()
            .filter(|card| cards_to_remove.contains(card))
            .cloned()
            .collect();

        self.deck.retain(|card| !cards_to_remove.contains(card));
        removed_cards
    }

    /// Searches the deck and removes cards within provided set of ranks/values
    ///
    /// Returns back a list of cards that were removed from the deck. Duplicates can be present in
    /// the returned vector if duplicates existed in the deck.
    pub fn strip_ranks(&mut self, ranks_to_remove: &HashSet<Value>) -> Vec<Card> {
        let removed_cards = self
            .deck
            .iter()
            .filter(|card| ranks_to_remove.contains(&card.value))
            .cloned()
            .collect();

        self.deck
            .retain(|card| !ranks_to_remove.contains(&card.value));

        removed_cards
    }

    /// Searches the deck and removes cards within provided set of suits
    ///
    /// Returns back a list of cards that were removed from the deck. Duplicates can be present in
    /// the returned vector if duplicates existed in the deck.
    pub fn strip_suits(&mut self, suits_to_remove: &HashSet<Suit>) -> Vec<Card> {
        let removed_cards = self
            .deck
            .iter()
            .filter(|card| suits_to_remove.contains(&card.suit))
            .cloned()
            .collect();

        self.deck
            .retain(|card| !suits_to_remove.contains(&card.suit));

        removed_cards
    }

    /// Adds the inputted cards into the muck
    ///
    /// This is primarily important if reshuffling the muck can occur.
    pub fn muck_cards(&mut self, mut cards: Vec<Card>) {
        self.muck.append(&mut cards);
    }

    /// Checks to see if there are enough cards in the deck to deal
    ///
    /// Returns true if there are enough cards, false otherwise.
    pub fn check_deal_cards(&self, cards_to_deal: usize, include_muck: bool) -> bool {
        let mut total_cards = self.deck.len();
        if include_muck {
            total_cards = self.muck.len();
        }
        total_cards >= cards_to_deal
    }

    /// Deals `n` cards out from the CardDeck
    ///
    /// If there is not enough cards remaining in the deck, it will reshuffle the mucked card back
    /// into the deck and redeal them out. If there are no more cards left, this method will return
    /// None. The method also returns
    ///
    /// Examples
    /// ```rust
    /// use playing_cards::core::{Card, CardDeck};
    /// use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
    ///
    /// let mut player_hands: Vec<Vec<Card>> = Vec::new();
    ///
    /// let mut deck: CardDeck = Default::default();
    /// deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());
    ///
    /// for i in 0..10 {
    ///     if let Some(hand) = deck.deal_cards(2, false) { // 2 cards per player would require 20 cards
    ///         player_hands.push(hand);
    ///     } else {
    ///         unreachable!("Ran out of cards!");
    ///     }
    /// }
    ///
    /// println!("{:?}", player_hands);
    /// ```
    ///
    /// ```rust should_panic
    /// use playing_cards::core::{Card, CardDeck};
    /// use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
    ///
    /// let mut player_hands: Vec<Vec<Card>> = Vec::new();
    ///
    /// let mut deck: CardDeck = Default::default();
    /// deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());
    ///
    /// for i in 0..10 {
    ///     if let Some(hand) = deck.deal_cards(6, false) { // 6 cards per player would require 60 cards, but there's only 52
    ///         player_hands.push(hand);
    ///     } else {
    ///         panic!("Ran out of cards!");
    ///     }
    /// }
    /// ```
    pub fn deal_cards(&mut self, cards_to_deal: usize, include_muck: bool) -> Option<Vec<Card>> {
        if !self.check_deal_cards(cards_to_deal, include_muck) {
            return None;
        }
        let mut cards_dealt: Vec<Card> = Vec::new();
        for _ in 0..cards_to_deal {
            if let Some(s) = self.next() {
                cards_dealt.push(s);
            }
        }

        Some(cards_dealt)
    }

    /// Draws `n` cards out from the CardDeck
    ///
    /// The definition of drawing in this case means to discard and replace cards. This function
    /// can take any number of discard cards with the help of `muck_cards()` and then simply
    /// invokes `deal_cards()` to deal `n` cards out of the deck.
    pub fn draw_cards(
        &mut self,
        cards_to_deal: usize,
        discard_cards: Option<Vec<Card>>,
        include_muck: bool,
    ) -> Option<Vec<Card>> {
        if !self.check_deal_cards(
            cards_to_deal
                - discard_cards
                    .as_ref()
                    .map_or(0, |v| if include_muck { v.len() } else { 0 }),
            include_muck,
        ) {
            return None;
        }
        if let Some(c) = discard_cards {
            self.muck_cards(c);
        }

        self.deal_cards(cards_to_deal, include_muck)
    }

    /// Reshuffles the muck and inserts those cards into the deck
    ///
    /// The muck will be placed behind the remaining cards in the deck.
    ///
    /// Similar to `shuffle()`, the `rng` argument is a PRNG whose type implements the `RngCore`
    /// trait. The use can decide whether the passed PRNG value is to be seeded from a set value or
    /// from entropy.
    pub fn reshuffle_muck(&mut self, rng: &mut dyn RngCore) {
        Self::shuffle_cards(&mut self.muck, rng);

        self.muck.append(&mut self.deck);
        self.deck = self.muck.to_owned();
        self.muck = Vec::new();
    }
}

impl Iterator for CardDeck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.deck.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Value;
    use super::*;
    use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256PlusPlus};
    use rayon::prelude::*;
    use std::iter::Iterator;

    #[test]
    fn test_deck_same_seed() {
        let mut seed_bytes = Vec::from(233_i32.to_le_bytes());
        seed_bytes.extend_from_slice(&[0u8; 28]);
        let mut d1 = CardDeck::new(Some(&mut Xoshiro256PlusPlus::from_seed(
            seed_bytes.as_mut_slice().try_into().unwrap(),
        )));
        let mut d2 = CardDeck::new(Some(&mut Xoshiro256PlusPlus::from_seed(
            seed_bytes.as_slice().try_into().unwrap(),
        )));

        are_decks_equal(&mut d1, &mut d2);
    }

    fn are_decks_equal(d1: &mut CardDeck, d2: &mut CardDeck) {
        let mut both_decks = Iterator::zip(d1, d2);
        for i in 0..52 {
            // checks all cards
            let both_cards = both_decks.next();

            assert_ne!(both_cards, None);

            if let Some((c1, c2)) = both_cards {
                assert_eq!(
                    c1, c2,
                    "Cards at index {} are not equal ({} != {})",
                    i, c1, c2
                );
            }
        }

        // then check if there is any extra cards over 52
        assert_eq!(both_decks.next(), None);
    }

    #[test]
    fn test_strip_spcific_cards() {
        let cards = Card::vec_from_str("2h5dAsAdKdJc3h8d").expect("Failed parsing card string");
        let mut deck: CardDeck = CardDeck::new_custom_deck(cards, None);
        deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());

        let cards_to_remove = HashSet::from_iter(
            Card::vec_from_str("5d2h8d3h")
                .expect("Failed parsing card string")
                .iter()
                .cloned(),
        );
        let actual_cards_removed = deck.strip_cards(&cards_to_remove);

        for c in actual_cards_removed {
            assert!(
                cards_to_remove.contains(&c),
                "{:?} is not inside expected list of cards removed: {:?}",
                c,
                cards_to_remove
            );
        }

        for c in deck {
            assert!(!cards_to_remove.contains(&c), "{:?} was not removed from deck. The following cards were to have been removed: {:?}", c, cards_to_remove);
        }
    }

    #[test]
    fn test_strip_ranks() {
        let cards = Card::vec_from_str("2h5dAsAdKdJc3h8d").expect("Failed parsing card string");
        let mut deck: CardDeck = CardDeck::new_custom_deck(cards, None);
        deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());

        let ranks_to_remove = HashSet::from([Value::Ace, Value::Two, Value::Three]);
        let actual_cards_removed = deck.strip_ranks(&ranks_to_remove);

        for c in actual_cards_removed {
            assert!(
                ranks_to_remove.contains(&c.value),
                "{:?} is not inside expected list of ranks removed: {:?}",
                c,
                ranks_to_remove
            );
        }

        for c in deck {
            assert!(!ranks_to_remove.contains(&c.value), "{:?} was not removed from deck. The following ranks were to have been removed: {:?}", c, ranks_to_remove);
        }
    }

    #[test]
    fn test_strip_suits() {
        let cards = Card::vec_from_str("2h5dAsAdKdJc3h8d").expect("Failed parsing card string");
        let mut deck: CardDeck = CardDeck::new_custom_deck(cards, None);
        deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());

        let suits_to_remove = HashSet::from([Suit::Spade, Suit::Diamond]);
        let actual_cards_removed = deck.strip_suits(&suits_to_remove);

        for c in actual_cards_removed {
            assert!(
                suits_to_remove.contains(&c.suit),
                "{:?} is not inside expected list of cards removed: {:?}",
                c,
                suits_to_remove
            );
        }

        for c in deck {
            assert!(!suits_to_remove.contains(&c.suit), "{:?} was not removed from deck. The following suits were to have been removed: {:?}", c, suits_to_remove);
        }
    }

    // This test relies on random entropy seeding. By the very nature of random numbers and normal
    // curves, there will be a subset of runs that will fail since the actual percentage lands
    // outside if the bounds of the expected percentage (+/- 0.2%).
    #[test]
    #[ignore]
    fn test_monte_carlo_2kings_adjacent() {
        let iters = 150000;

        let count: i32 = (0..iters)
            .into_par_iter()
            .map(|_| {
                let mut deck: CardDeck = Default::default();

                deck.shuffle(&mut Xoshiro256PlusPlus::from_entropy());

                if are_2kings_adjacent(&mut deck) {
                    1
                } else {
                    0
                }
            })
            .sum();

        let expected_prob = 1201.0 / 5525.0; // 1 - ((49! / (49-4)! * 48!) / 52!)
        let actual_prob = (count as f64) / (iters as f64);
        let epsilon = 0.002; // within a percentage of error of the actual
        assert!((actual_prob - expected_prob).abs() < epsilon, "Probability did not fall within {} of expected probability with {} iterations. Expected: {} (Actual: {})", epsilon, iters, expected_prob, actual_prob);
    }

    fn are_2kings_adjacent(deck: &mut CardDeck) -> bool {
        let mut was_previous_king = false;
        while let Some(c) = deck.next() {
            if c.value == Value::King {
                if was_previous_king {
                    return true;
                }
                was_previous_king = true;
            } else {
                was_previous_king = false;
            }
        }

        false
    }
}
