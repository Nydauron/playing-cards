use crate::{core::Card, poker::Rank};

use once_cell::sync::Lazy;

extern crate bincode;

pub static LOOKUP_TABLE: Lazy<Vec<i32>> = Lazy::new(|| {
    let buf = include_bytes!(concat!(env!("OUT_DIR"), "/src/poker/evaluators/HandRanks.dat"));
    // Here is something to help improve this:
    // We use abomonation (https://docs.rs/abomonation/latest/abomonation/index.html) to help encode and decode the struct
    // When building the library, we have the struct be generated then encoded and then write all of the bytes to a .dat file
    // Then, we use include_bytes!() and decode the struct

    let mut lookup_table: Vec<i32> = bincode::deserialize(buf).unwrap();
    lookup_table.shrink_to_fit();
    lookup_table
});

/// A Trait definition for all poker evaluators.
pub trait Evaluator {

    /// Evaluates a hand for one player.
    fn evaluate_hand(&self, player_hand: &Vec<Card>, board: &Vec<Card>) -> Result<Rank, &str>;
}

/// This function allows for the 2+2 lookup table to be loaded in. This function only needs to be
/// called once, and calling it multiple times does not have any significant performance penalties.
///
/// This exists due to the fact that the first inital load of the table takes about 8 seconds to
/// initialize using `lazy_static`. In an ideal world, the table should be baked into static
/// memory.
pub fn init_lookup_table() {
    print!("Loading LOOKUP_TABLE ... ");
    Lazy::force(&LOOKUP_TABLE);
    println!("Done!");
}
