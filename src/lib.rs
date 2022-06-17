#[macro_use]
extern crate num_derive;

mod core;

#[cfg(feature = "poker")]
mod poker;

#[cfg(test)]
mod tests {}
