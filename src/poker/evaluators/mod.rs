//! Contains all hand evaluators.

mod evaluator;
pub use self::evaluator::{Evaluator, init_lookup_table};
pub (super) use self::evaluator::LOOKUP_TABLE; 

mod high_evaluator;
pub use self::high_evaluator::HighEvaluator;

mod low_evaluator;
pub use self::low_evaluator::LowEvaluator;

mod omaha_hi_evaluator;
pub use self::omaha_hi_evaluator::OmahaHighEvaluator;
