#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use scotland_yard_common::STARTING_FIELDS;

use crate::state::GameState;

mod detectives;
mod mrx;
mod state;

fn main() {
    let state = GameState::new(STARTING_FIELDS[0], &[STARTING_FIELDS[1]]).unwrap();
}
