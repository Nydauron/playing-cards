extern crate num;
use num::traits::FromPrimitive;
use strum_macros::EnumIter;

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

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, EnumIter)]
pub enum Suit {
    Heart = 0,
    Club = 1,
    Diamond = 2,
    Spade = 3,
}

impl Suit {
    pub fn get_char(& self) -> char {
        match self {
            Self::Heart => 'h',
            Self::Club => 'c',
            Self::Diamond => 'd',
            Self::Spade => 's',
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

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl From<i32> for Card {
    fn from(s: i32) -> Card {
        Card {
            value: Value::try_from((s - 1) / 4).unwrap(),
            suit: Suit::try_from((s - 1) % 4).unwrap(),
        }
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