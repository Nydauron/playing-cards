use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub struct Rank {
    pub strength: u32,
    pub hand_rank: u16,
    pub sub_rank: u16,
    pub description: Option<String>,
}

impl Rank {
    pub fn get_strength(&self) -> u32 {
        self.strength
    }

    pub fn get_hand_rank(&self) -> u16 {
        self.hand_rank
    }

    pub fn get_sub_rank(&self) -> u16 {
        self.sub_rank
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}
