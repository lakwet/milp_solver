/// Fundamental theorem of linear programming:
/// Any linear program, given in standard form, either
/// 1. has an optimal solution with a finite objective value.
/// 2. is infeasible or
/// 3. is unbounded
/// (Introduction to algorithm, 2009, Cormen, Leiserson, et al)
pub mod algo;
#[allow(non_snake_case)]
mod forms;
pub mod shared;
#[allow(non_snake_case)]
#[cfg(test)]
mod tests;
mod types;

pub use forms::builder::StandardFormBuilder;
pub use forms::slack::SlackFormLP;
pub use forms::standard::StandardFormLP;
