extern crate num;
use num::traits::FromPrimitive;
use strum_macros::EnumIter;
use std::str::FromStr;

/// An enum representation of the rank of a card
///
/// Each value corresponds to the rank strength.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, EnumIter)]
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
    /// Gets the associated character for the Value.
    ///
    /// This is typically used to parse a Value into a string format for users like printing
    /// shortened ASCII card representations (e.g. As for the Ace of spades, 5d for the 5 of
    /// diamonds).
    pub fn get_char(& self) -> char {
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

    /// Attempts to parse a character and returns the associated Value.
    ///
    /// The function will return back None if the input character is not any of the mapped
    /// characters.
    pub fn from_char(c: char) -> Option<Value> {
        match c {
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
            _ => None
        }
    }

    /// Attempts to parse an interger to a Value.
    ///
    /// Returns back None if the number does not fall within `[0, 13)`.
    pub fn from_int(u: u64) -> Option<Value> {
        match u {
            0 => Some(Self::Two),
            1 => Some(Self::Three),
            2 => Some(Self::Four),
            3 => Some(Self::Five),
            4 => Some(Self::Six),
            5 => Some(Self::Seven),
            6 => Some(Self::Eight),
            7 => Some(Self::Nine),
            8 => Some(Self::Ten),
            9 => Some(Self::Jack),
            10 => Some(Self::Queen),
            11 => Some(Self::King),
            12 => Some(Self::Ace),
            _ => None
        }
    }

    /// Returns a prettified string of the Value.
    ///
    /// These strings are meant for end-users and can also be used for printing
    /// hand ranks.
    pub fn get_readable_string(& self) -> String {
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


impl Into<char> for Value {
    fn into(self) -> char {
        self.get_char()
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
/// Numerical value is just for distinction and each suit has equal strength
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, EnumIter)]
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
    pub fn get_char(& self) -> char {
        match self {
            Self::Heart => 'h',
            Self::Club => 'c',
            Self::Diamond => 'd',
            Self::Spade => 's',
        }
    }

    /// Attempts to parse a character and returns the associated Suit.
    ///
    /// The function will return back None if the input character is not any of the mapped
    /// characters.
    pub fn from_char(c: char) -> Option<Suit> {
        match c {
            'h' => Some(Self::Heart),
            'c' => Some(Self::Club),
            'd' => Some(Self::Diamond),
            's' => Some(Self::Spade),
            _ => None
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

impl Into<char> for Suit {
    fn into(self) -> char {
        self.get_char()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c: char = self.get_char();
        write!(f, "{}", c)
    }
}

/// A structural representation of a playing card.
#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    /// The Value of the Card
    pub value: Value,
    /// The Suit of the Card
    pub suit: Suit,
}

impl Card {
    /// Takes in a string and returns back a vector of Cards.
    ///
    /// This can be used to quickly static hands that can be evaluated for testing.
    pub fn vec_from_str(s: &str) -> Result<Vec<Card>, &str> {
        if s.len() % 2 != 0 {
            return Err("not a valid string");
        }

        let mut cards: Vec<Card> = Vec::new();
        for i in (0..s.len()).step_by(2) {
            let c = Card::from_str(s.get(i..i+2).unwrap()).unwrap();
            cards.push(c);
        }

        Ok(cards)
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

impl FromStr for Card {
    type Err = &'static str;
    fn from_str(s: &'_ str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err("String is not of length 2");
        }

        let mut chars = s.chars();
        Ok(Card {
            value: Value::try_from(chars.next().unwrap()).unwrap(),
            suit: Suit::try_from(chars.next().unwrap()).unwrap(),
        })
    }
}

impl Into<i32> for Card {
    fn into(self) -> i32 {
        (self.value as i32) * 4 + self.suit as i32 + 1
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
}