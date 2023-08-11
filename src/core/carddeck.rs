use getrandom;
use std::{collections::HashSet, io::Error};

extern crate rand;

use rand::seq::SliceRandom;
use rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use strum::IntoEnumIterator;

use super::{Card, Suit, Value};

/// A deck of cards.
///
/// This deck will contain 52 distinct cards upon initalization. To ensure uniform randomness,
/// mersenne twisters are used when the deck is intialized and everytime when the muck is
/// reshuffled back in.
///
/// Example
/// ```rust
/// use playing_cards::core::CardDeck;
///
/// let mut deck: CardDeck = Default::default();
/// deck.shuffle(None);
///
/// let hand = deck.deal_cards(2, false);
///
/// println!("{:?}", hand.unwrap()); // Two random cards from the deck
/// ```
pub struct CardDeck {
    deck: Vec<Card>,
    seed: Option<[u8; 32]>,
    muck: Vec<Card>,
}

impl Default for CardDeck {
    fn default() -> Self {
        Self::create_unshuffled_deck()
    }
}

impl CardDeck {
    /// Creates a new shuffled or unshuffled CardDeck.
    ///
    /// This method is a way to create deterministic deck for random but predictiable outcomes.
    /// Please note that this method will attempt to shuffle the deck if a seed is provided, but if
    /// shuffling fails, `new_with_seed()` will return an error.
    ///
    /// If no seed is provided, then an unshuffled deck is returned. This is identical to the
    /// behavior of `Default::default()`.
    ///
    /// Example
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
    ///
    /// for _ in 0..10 {
    ///     let mut seed_bytes = Vec::from(1337_u32.to_ne_bytes());
    ///     seed_bytes.extend_from_slice(&[0u8; 28]);
    ///     let mut deck = CardDeck::new(Some(seed_bytes.as_slice().try_into().unwrap())).unwrap();
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
    ///
    /// for i in 0..10 {
    ///     let mut seed_bytes = Vec::from((i as u32).to_ne_bytes());
    ///     seed_bytes.extend_from_slice(&[0u8; 28]);
    ///     let mut deck = CardDeck::new(Some(seed_bytes.as_slice().try_into().unwrap())).unwrap();
    ///
    ///     // Each line should be different from one another, but if you rerun this code again,
    ///     // it will print out the exact 10 lines again.
    ///     let hand = deck.deal_cards(5, false);
    ///     println!("{:?}", hand.unwrap());
    /// }
    /// ```
    ///
    /// If you do provide a seed to `new()`, the cards within the deck can be predicted if the seed
    /// generation is predictable (e.g. incrementing the seed by one, using unix time). It is
    /// better to use `new()` in these cases since the entropy from the system cannot be replicated
    /// across systems easily unless the seed generated is shared.
    pub fn new(seed: Option<[u8; 32]>) -> Result<CardDeck, Error> {
        let mut deck = Self::create_unshuffled_deck();

        if let Some(_) = seed {
            deck.shuffle(seed)?;
        }

        Ok(deck)
    }

    /// Creates a new CardDeck with provided `cards`.
    ///
    /// Useful if a standard 52-card deck does not fulfill your needs.
    ///
    /// Will attempt to shuffle deck if a seed is provided. An error will return if shuffling
    /// fails. If no seed is provided, the deck remains unshuffled.
    pub fn new_custom_deck(cards: Vec<Card>, seed: Option<[u8; 32]>) -> Result<Self, Error> {
        let mut deck = CardDeck {
            deck: cards,
            seed: seed,
            muck: Vec::new(),
        };

        if let Some(_) = seed {
            deck.shuffle(seed)?;
        }

        Ok(deck)
    }

    fn create_unshuffled_deck() -> CardDeck {
        let mut d: Vec<Card> = Vec::with_capacity(52);

        for s in Suit::iter() {
            for v in Value::iter() {
                d.push(Card { value: v, suit: s });
            }
        }

        CardDeck {
            deck: d,
            seed: None,
            muck: Vec::new(),
        }
    }

    /// Shuffles the deck.
    ///
    /// An optional seed can be provided if the deck should be shuffled with a specific seed. If no
    /// seed is provided, then system entropy is sampled for a random seed.
    pub fn shuffle(&mut self, seed: Option<[u8; 32]>) -> Result<(), Error> {
        self.seed = Some(Self::shuffle_cards(&mut self.deck, seed)?);
        Ok(())
    }

    fn shuffle_cards(cards: &mut Vec<Card>, seed: Option<[u8; 32]>) -> Result<[u8; 32], Error> {
        let mut rng;
        let mut seed_used;
        match seed {
            Some(seed) => seed_used = seed,
            None => {
                seed_used = [0u8; 32];
                getrandom::getrandom(&mut seed_used)?;
            }
        }
        rng = Xoshiro256PlusPlus::from_seed(seed_used);
        cards.shuffle(&mut rng);
        Ok(seed_used)
    }

    /// Gets the mersenne twister seed of the CardDeck.
    pub fn get_seed(&self) -> Option<[u8; 32]> {
        self.seed
    }

    /// Searches the deck and removes cards within provided set of cards.
    ///
    /// Returns back a list of cards that were removed from the deck. Duplicates can be present in
    /// the returned vector if duplicates existed in the deck.
    pub fn strip_cards(&mut self, cards_to_remove: &HashSet<Card>) -> Vec<Card> {
        let removed_cards: Vec<Card> = self
            .deck
            .clone()
            .into_iter()
            .filter(|card| cards_to_remove.contains(card))
            .collect();

        self.deck.retain(|card| !cards_to_remove.contains(card));
        removed_cards
    }

    /// Searches the deck and removes cards within provided set of ranks/values.
    ///
    /// Returns back a list of cards that were removed from the deck. Duplicates can be present in
    /// the returned vector if duplicates existed in the deck.
    pub fn strip_ranks(&mut self, ranks_to_remove: &HashSet<Value>) -> Vec<Card> {
        let removed_cards: Vec<Card> = self
            .deck
            .clone()
            .into_iter()
            .filter(|card| ranks_to_remove.contains(&card.value))
            .collect();

        self.deck
            .retain(|card| !ranks_to_remove.contains(&card.value));

        removed_cards
    }

    /// Searches the deck and removes cards within provided set of suits.
    ///
    /// Returns back a list of cards that were removed from the deck. Duplicates can be present in
    /// the returned vector if duplicates existed in the deck.
    pub fn strip_suits(&mut self, suits_to_remove: &HashSet<Suit>) -> Vec<Card> {
        let removed_cards: Vec<Card> = self
            .deck
            .clone()
            .into_iter()
            .filter(|card| suits_to_remove.contains(&card.suit))
            .collect();

        self.deck
            .retain(|card| !suits_to_remove.contains(&card.suit));

        removed_cards
    }

    /// Adds the inputted cards into the muck.
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

    /// Deals `n` cards out from the CardDeck.
    ///
    /// If there is not enough cards remaining in the deck, it will reshuffle the mucked card back
    /// into the deck and redeal them out. If there are no more cards left, this method will return
    /// None. The method also returns
    ///
    /// Examples
    /// ```rust
    /// use playing_cards::core::{Card, CardDeck};
    ///
    /// let mut player_hands: Vec<Vec<Card>> = Vec::new();
    ///
    /// let mut deck: CardDeck = Default::default();
    /// deck.shuffle(None);
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
    ///  use playing_cards::core::{Card, CardDeck};
    ///
    /// let mut player_hands: Vec<Vec<Card>> = Vec::new();
    ///
    /// let mut deck: CardDeck = Default::default();
    /// deck.shuffle(None);
    ///
    /// for i in 0..10 {
    ///     if let Some(hand) = deck.deal_cards(6, false) { // 6 cards per player would require 60 cards, but there's only 52
    ///         player_hands.push(hand);
    ///     } else {
    ///         panic!("Ran out of cards!");
    ///     }
    /// }
    ///
    /// unreachable!();
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

    /// Draws `n` cards out from the CardDeck.
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
                    .clone()
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

    /// Reshuffles the muck and inserts those cards into the deck.
    ///
    /// The muck will be placed behind the remaining cards in the deck.
    ///
    /// Similar to `shuffle()` this funtion takes in an optional seed if a specific seed is
    /// desired. If no seed is provided, a seed will be sampled from entropy.
    pub fn reshuffle_muck(&mut self, seed: Option<[u8; 32]>) -> Result<(), Error> {
        Self::shuffle_cards(&mut self.muck, seed)?;

        self.muck.append(&mut self.deck);
        self.deck = self.muck.to_owned();
        self.muck = Vec::new();

        Ok(())
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
    use rayon::prelude::*;
    use std::iter::Iterator;

    #[test]
    fn test_deck_same_seed() {
        let mut seed_bytes = Vec::from(233_i32.to_le_bytes());
        seed_bytes.extend_from_slice(&[0u8; 28]);
        let mut d1 = CardDeck::new(Some(seed_bytes.as_slice().try_into().unwrap())).unwrap();
        let mut d2 = CardDeck::new(Some(seed_bytes.as_slice().try_into().unwrap())).unwrap();

        are_decks_equal(&mut d1, &mut d2);
    }

    fn are_decks_equal(d1: &mut CardDeck, d2: &mut CardDeck) {
        assert_eq!(d1.seed, d2.seed);
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
    fn test_get_seed() {
        let mut expected_seed = Vec::from(233_i32.to_le_bytes());
        expected_seed.extend_from_slice(&[0u8; 28]);
        let d = CardDeck::new(Some(expected_seed.as_slice().try_into().unwrap())).unwrap();

        assert_eq!(Vec::from(d.get_seed().unwrap()), expected_seed);
    }

    #[test]
    fn test_strip_spcific_cards() {
        let cards = Card::vec_from_str("2h5dAsAdKdJc3h8d").expect("Failed parsing card string");
        let mut deck = CardDeck::new_custom_deck(cards, None).expect("Deck could not be created");
        deck.shuffle(None).expect("Shuffle failed");

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
        let mut deck = CardDeck::new_custom_deck(cards, None).expect("Deck could not be created");
        deck.shuffle(None).expect("Shuffle failed");

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
        let mut deck = CardDeck::new_custom_deck(cards, None).expect("Deck could not be created");
        deck.shuffle(None).expect("Shuffle failed");

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

                deck.shuffle(None)
                    .expect("Problem occured when shuffling the deck");

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
