#![deny(clippy::all)]
#![deny(clippy::dbg_macro)]

mod evaluator;
mod gate;
mod requirement;
mod token;

pub use evaluator::Evaluator;
pub use gate::Gate;
pub use requirement::Requirement;
pub use token::TokenTree;

pub type TerminalId = usize;
