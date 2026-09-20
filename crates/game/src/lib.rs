#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod detectives;
mod mr_x;
mod state;

pub use detectives::*;
pub use mr_x::*;
pub use state::*;
