#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use std::error::Error;

use scotland_yard_common::{MrXTicket, STARTING_FIELDS, Station};
use scotland_yard_game::{Game, MrXMove, SingleMrXMove};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(STARTING_FIELDS[0], &[STARTING_FIELDS[1]])?;

    game.move_mr_x(MrXMove {
        first: SingleMrXMove {
            destination: Station::new(4).unwrap(),
            ticket: MrXTicket::Taxi,
        },
        second: None,
    })?;

    dbg!(game.state());

    Ok(())
}
