//! Contains all hand evaluators.

mod evaluator;
pub use self::evaluator::{Evaluator, init_lookup_table};
pub (super) use self::evaluator::LOOKUP_TABLE; 

mod high_evaluator;
pub use self::high_evaluator::HighEvaluator;
