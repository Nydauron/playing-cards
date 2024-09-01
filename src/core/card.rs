use num_traits::FromPrimitive;
use std::str::FromStr;
use strum_macros::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An enum representation of the rank of a card
///
/// Each value corresponds to the rank strength.
///
/// With the `serde` feature enabled, this struct also implements serde's `Serialize` and
/// `Deserialize` traits.
#[allow(missing_docs)]
#[derive(
    Debug, Clone, Copy, FromPrimitive, ToPrimitive, EnumIter, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Value {
    /// Gets the associated character for the Value
    ///
    /// This is typically used to parse a Value into a string format for users like printing
    /// shortened ASCII card representations (e.g. As for the Ace of spades, 5d for the 5 of
    /// diamonds).
    pub fn get_char(&self) -> char {
        match self {
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Ten => 'T',
            Self::Jack => 'J',
            Self::Queen => 'Q',
            Self::King => 'K',
            Self::Ace => 'A',
        }
    }

    /// Parses a character, returning the corresponding Value if valid.
    ///
    /// Returns `None` for characters not representing a Value. This function is case-insensitive.
    pub fn from_char(c: char) -> Option<Value> {
        match c.to_ascii_uppercase() {
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'T' => Some(Self::Ten),
            'J' => Some(Self::Jack),
            'Q' => Some(Self::Queen),
            'K' => Some(Self::King),
            'A' => Some(Self::Ace),
            _ => None,
        }
    }

    /// Returns a prettified string of the Value
    ///
    /// These strings are meant for end-users and can also be used for printing
    /// hand ranks.
    pub fn get_readable_string(&self) -> String {
        match self {
            Self::Two => "2".to_string(),
            Self::Three => "3".to_string(),
            Self::Four => "4".to_string(),
            Self::Five => "5".to_string(),
            Self::Six => "6".to_string(),
            Self::Seven => "7".to_string(),
            Self::Eight => "8".to_string(),
            Self::Nine => "9".to_string(),
            Self::Ten => "10".to_string(),
            Self::Jack => "Jack".to_string(),
            Self::Queen => "Queen".to_string(),
            Self::King => "King".to_string(),
            Self::Ace => "Ace".to_string(),
        }
    }

    /// Returns the associated Cactus-Kev prime
    ///
    /// Useful for building the original or variants of the Cactus-Kev evaluator.
    pub fn get_cactus_kev_prime(&self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 5,
            Self::Five => 7,
            Self::Six => 11,
            Self::Seven => 13,
            Self::Eight => 17,
            Self::Nine => 19,
            Self::Ten => 23,
            Self::Jack => 29,
            Self::Queen => 31,
            Self::King => 37,
            Self::Ace => 41,
        }
    }

    /// Returns true if the card value is of a face card (Jack, Queen, King)
    pub fn is_face_card(&self) -> bool {
        matches!(self, Self::Jack | Self::Queen | Self::King)
    }
}

impl TryFrom<i32> for Value {
    type Error = i32;
    fn try_from(s: i32) -> Result<Self, Self::Error> {
        match Value::from_i32(s) {
            Some(val) => Ok(val),
            None => Err(s),
        }
    }
}

impl TryFrom<char> for Value {
    type Error = char;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match Value::from_char(s) {
            Some(val) => Ok(val),
            None => Err(s),
        }
    }
}

impl From<Value> for char {
    fn from(value: Value) -> Self {
        value.get_char()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c: char = self.get_char();
        write!(f, "{}", c)
    }
}

/// An enum representation of the suit of a card
///
/// Numerical value is just for distinction and each suit has equal strength.
///
/// With the `serde` feature enabled, this struct also implements serde's `Serialize` and
/// `Deserialize` traits.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, EnumIter, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Suit {
    Heart = 0,
    Club = 1,
    Diamond = 2,
    Spade = 3,
}

impl Suit {
    /// Gets the associated character for the Suit
    ///
    /// This is typically used to parse a Suit into a string format for users like printing
    /// shortened ASCII card representations (e.g. As for the Ace of spades, 5d for the 5 of
    /// diamonds).
    pub fn get_char(&self) -> char {
        match self {
            Self::Heart => 'h',
            Self::Club => 'c',
            Self::Diamond => 'd',
            Self::Spade => 's',
        }
    }

    /// Parses a character, returning the corresponding Suit if valid.
    ///
    /// The input is either a case-insensitive letter, or the UTF-8 character representing the suit.
    /// Returns `None` for characters not representing a Suit.
    pub fn from_char(c: char) -> Option<Suit> {
        match c.to_ascii_lowercase() {
            'h' | '♥' => Some(Self::Heart),
            'c' | '♣' => Some(Self::Club),
            'd' | '♦' => Some(Self::Diamond),
            's' | '♠' => Some(Self::Spade),
            _ => None,
        }
    }
}

impl TryFrom<i32> for Suit {
    type Error = i32;
    fn try_from(s: i32) -> Result<Self, Self::Error> {
        match Suit::from_i32(s) {
            Some(val) => Ok(val),
            None => Err(s),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = char;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match Suit::from_char(s) {
            Some(val) => Ok(val),
            None => Err(s),
        }
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        value.get_char()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match *self {
            Suit::Heart => '♥',
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Spade => '♠',
        };
        write!(f, "{}", symbol)
    }
}

/// A structural representation of a playing card
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "String"),
    serde(into = "String")
)]
pub struct Card {
    /// The Value of the Card
    pub value: Value,
    /// The Suit of the Card
    pub suit: Suit,
}

impl Card {
    /// Takes in a string and returns back a vector of Cards
    ///
    /// This can be used to quickly static hands that can be evaluated for testing.
    pub fn vec_from_str(s: &str) -> Result<Vec<Card>, &str> {
        if s.len() % 2 != 0 {
            return Err("not a valid string");
        }

        let mut cards: Vec<Card> = Vec::new();
        for i in (0..s.len()).step_by(2) {
            let c = Card::from_str(s.get(i..i + 2).unwrap()).unwrap();
            cards.push(c);
        }

        Ok(cards)
    }

    /// Turns card into integer
    ///
    /// This is typically used for when traversing the lookup table.
    pub fn to_int(&self) -> i32 {
        ((self.value as i32) * 4) + (self.suit as i32) + 1
    }

    /// Calculates the Catus-Kev bit pattern for the card
    ///
    /// This can be useful for building custom hand evaluators. For poker-related hand evaluators,
    /// please see the poker module.
    pub fn calculate_bit_pattern(&self) -> u32 {
        let mut bit_pattern: u32 = 0;
        bit_pattern |= 1 << (16 + self.value as u32);
        bit_pattern |= 1
            << (12
                + match self.suit {
                    Suit::Heart => 1,
                    Suit::Club => 3,
                    Suit::Diamond => 2,
                    Suit::Spade => 0,
                });
        bit_pattern |= (self.value as u32) << 8;
        bit_pattern |= self.value.get_cactus_kev_prime() as u32;

        bit_pattern
    }

    /// Returns true if the card is a face card (Jack, Queen, King)
    pub fn is_face_card(&self) -> bool {
        self.value.is_face_card()
    }
}

impl From<i32> for Card {
    fn from(s: i32) -> Card {
        Card {
            value: Value::try_from((s - 1) / 4).unwrap(),
            suit: Suit::try_from((s - 1) % 4).unwrap(),
        }
    }
}

impl TryFrom<String> for Card {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.chars().count() != 2 {
            return Err(format!(
                r#"Card string "{}" is not exactly a length of 2"#,
                s
            ));
        }

        let mut chars = s.chars();

        let value = Value::try_from(chars.next().unwrap());
        if value.is_err() {
            return Err(format!(
                r#"Card value "{}" was not a valid character"#,
                value.unwrap_err()
            ));
        }
        let value = value.unwrap();

        let suit = Suit::try_from(chars.next().unwrap());
        if suit.is_err() {
            return Err(format!(
                r#"Card suit "{}" was not a valid character"#,
                suit.unwrap_err()
            ));
        }
        let suit = suit.unwrap();

        Ok(Card { value, suit })
    }
}

impl From<Card> for String {
    fn from(c: Card) -> Self {
        c.to_string()
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &'_ str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl From<Card> for i32 {
    fn from(value: Card) -> Self {
        (value.value as i32) * 4 + value.suit as i32 + 1
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_card() {
        let card = Card {
            value: Value::Ace,
            suit: Suit::Spade,
        };
        let actual_int: i32 = card.into();
        assert_eq!(52, actual_int);
    }

    #[test]
    fn to_card() {
        let val = 52;

        let expected_card = Card {
            value: Value::Ace,
            suit: Suit::Spade,
        };

        assert_eq!(expected_card, Card::from(val));
    }

    #[test]
    #[should_panic]
    fn value_under_valid_card() {
        let val = 0;

        let _ = Card::from(val);
    }

    #[test]
    #[should_panic]
    fn value_over_valid_card() {
        let val = 53;

        let _ = Card::from(val);
    }

    #[test]
    fn conversion() {
        for card_str in ["AH", "ah"] {
            let card = Card::from_str(card_str).unwrap();
            assert_eq!(card.to_int(), 49);
        }
    }

    #[test]
    fn conversion_from_suit_symbols() {
        for index in 1..=52 {
            let card = Card::from(index);
            assert_eq!(Card::from_str(&card.to_string()).unwrap(), card);
        }
    }

    #[test]
    fn conversion_error() {
        assert_eq!(
            Card::from_str("xh").unwrap_err(),
            r#"Card value "x" was not a valid character"#
        );

        assert_eq!(
            Card::from_str("Ky").unwrap_err(),
            r#"Card suit "y" was not a valid character"#,
        );

        assert_eq!(
            Card::from_str("abc").unwrap_err(),
            r#"Card string "abc" is not exactly a length of 2"#,
        );
    }
}
