#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use std::error::Error;

use scotland_yard_common::{MrXTicket, STARTING_FIELDS, Station};
use scotland_yard_game::{Game, GameMove, MrXMove};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new(STARTING_FIELDS[0], &[STARTING_FIELDS[1]])?;

    game.transition(GameMove::MrX {
        mov: MrXMove {
            ticket: MrXTicket::Taxi,
            destination: const { Station::new(4).unwrap() },
        },
        use_double_ticket: false,
    })?;

    dbg!(game.state());

    Ok(())
}
