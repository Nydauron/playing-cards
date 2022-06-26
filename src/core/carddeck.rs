use std::io::{Cursor, Error};
use getrandom;
use byteorder::{BigEndian, ReadBytesExt};

extern crate sfmt;
extern crate rand;

use rand::seq::SliceRandom;
use rand_core::SeedableRng;

use strum::IntoEnumIterator;

use super::{Card, Value, Suit};

pub struct CardDeck {
    deck: Vec<Card>,
    seed: u64,
    mt:   sfmt::SFMT,
    muck: Vec<Card>,
}

impl CardDeck {
    // TODO: Add a way of creating different forms of short decks (e.g. A-7, 6-A)
    pub fn new() -> Result<CardDeck, Error> {
        let mut buf = [0u8; 8];
        let res = getrandom::getrandom(&mut buf);

        if let Err(e) = res {
            return Err(From::<getrandom::Error>::from(e));
        }

        let mut rdr = Cursor::new(buf);

        let seed = rdr.read_u64::<BigEndian>().unwrap();

        // Could do this, but idk if there is a way to get the seed
        // let mut mt = sfmt::SFMT::from_entropy();
        // CardDeck::new_with_mt(&mut mt)

        Ok(CardDeck::new_with_seed(seed))
    }

    pub fn new_with_seed(seed: u64) -> CardDeck {
        let mt = sfmt::SFMT::seed_from_u64(seed);

        CardDeck::new_with_mt(& mt, seed)
    }

    fn new_with_mt(mt: & sfmt::SFMT, seed: u64) -> CardDeck {
        let mut d: Vec<Card> = Vec::with_capacity(52);

        for s in Suit::iter() {
            for v in Value::iter() {
                d.push(Card{
                    value: v,
                    suit: s,
                });
            }
        }

        let mut deck: CardDeck = CardDeck{
            deck: d,
            seed: seed,
            mt: mt.to_owned(),
            muck: Vec::new(),
        };

        deck.shuffle();

        deck
    }

    fn shuffle(&mut self) {
        self.deck.shuffle(&mut self.mt);
    }

    pub fn get_seed(& self) -> u64 {
        self.seed
    }

    pub fn muck_cards(&mut self, mut cards: Vec<Card>) {
        self.muck.append(&mut cards);
    }

    pub fn check_deal_cards(& self, n: usize, m: usize) -> bool {
        self.deck.len() + self.muck.len() >= n - m
    }

    pub fn deal_cards(&mut self, n: usize) -> (Option<Vec<Card>>, bool) {
        let mut cards_to_deal: Vec<Card> = Vec::new();
        let mut was_deck_reshuffled = false;
        for _i in 0..n {
            if let Some(s) = self.next() {
                cards_to_deal.push(s);
            } else {
                self.reshuffle_muck();
                was_deck_reshuffled = true;
            }
        }

        if cards_to_deal.len() == n {
            (Some(cards_to_deal), was_deck_reshuffled)
        } else {
            (None, was_deck_reshuffled)
        }
    }

    pub fn draw_cards(&mut self, n: usize, discard_cards: Option<Vec<Card>>) -> (Option<Vec<Card>>, bool) {
        if let Some(c) = discard_cards {
            self.muck_cards(c);
        }

        self.deal_cards(n)
    }

    fn reshuffle_muck(&mut self) -> bool {
        if self.deck.len() == 0 {
            self.muck.shuffle(&mut self.mt);

            self.deck = self.muck.to_owned();
            self.muck = Vec::new();

            true
        } else {
            false
        }
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
    use super::*;
    use rayon::prelude::*;
    use std::iter::Iterator;
    use super::super::Value;

    #[test]
    fn test_deck_same_seed() {
        let mut d1 = CardDeck::new_with_seed(233);
        let mut d2 = CardDeck::new_with_seed(233);

        are_decks_equal(&mut d1,&mut d2);
    }

    fn are_decks_equal(d1: &mut CardDeck, d2: &mut CardDeck) {
        assert_eq!(d1.seed, d2.seed);
        let mut both_decks = Iterator::zip(d1, d2);
        for i in 0..52 { // checks all cards
            let both_cards = both_decks.next();

            assert_ne!(both_cards, None);

            if let Some((c1,c2)) = both_cards {
                assert_eq!(c1, c2, "Cards at index {} are not equal ({} != {})", i, c1, c2);
            }
        }

        // then check if there is any extra cards over 52
        assert_eq!(both_decks.next(), None);
    }

    #[test]
    fn test_get_seed() {
        let expected_seed = 0x879e280ef4749657;
        let d = CardDeck::new_with_seed(expected_seed);

        assert_eq!(d.get_seed(), expected_seed);
    }

    #[test]
    #[ignore]
    fn test_monte_carlo_2kings_adjacent() {
        let iters = 300000;

        let count : i32 = (0..iters).into_par_iter().map(|_| {
            let mut deck = CardDeck::new().unwrap();
            
            if are_2kings_adjacent(&mut deck) {
                1
            } else {
                0
            }
        })
        .sum();

        let expected_prob = 1201.0/5525.0; // 1 - ((49! / (49-4)! * 48!) / 52!)
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