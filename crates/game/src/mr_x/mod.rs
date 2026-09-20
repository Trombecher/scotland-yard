mod errors;
mod tickets;

pub use errors::*;
pub use tickets::*;

use std::fmt::Display;

use scotland_yard_common::{
    MrXTicket, Station,
    connections::{CONNECTIONS, Connection},
    content::DETECTIVE_COUNT_RANGE,
};

use crate::detectives::DetectiveState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SingleMrXMove {
    pub ticket: MrXTicket,
    pub destination: Station,
}

impl Display for SingleMrXMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "with {} to {}", self.ticket, self.destination)
    }
}

impl SingleMrXMove {
    #[must_use]
    pub fn connection_from_station(self, from: Station) -> Connection {
        Connection {
            from,
            kind: self.ticket.into(),
            to: self.destination,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MrXMove {
    pub first: SingleMrXMove,
    pub second: Option<SingleMrXMove>,
}

impl MrXMove {
    #[must_use]
    pub const fn destination(self) -> Station {
        if let Some(second) = self.second {
            second.destination
        } else {
            self.first.destination
        }
    }
}

pub struct MrXState {
    start: Station,
    moves: Vec<MrXMove>,
    remaining_tickets: RemainingMrXTickets,
}

impl MrXState {
    #[must_use]
    pub fn new(start: Station, detective_count: u8) -> Option<Self> {
        if !DETECTIVE_COUNT_RANGE.contains(&detective_count) {
            return None;
        }

        Some(Self {
            start,
            moves: Vec::new(),
            remaining_tickets: RemainingMrXTickets::new(detective_count),
        })
    }

    pub fn remaining_tickets(&self) -> RemainingMrXTickets {
        self.remaining_tickets
    }

    #[must_use]
    pub fn current_station(&self) -> Station {
        self.moves
            .last()
            .copied()
            .map_or(self.start, MrXMove::destination)
    }

    /// Tries to move Mr. X.
    ///
    /// # Errors
    ///
    /// If invariants are dissatisfied.
    pub(crate) fn move_by(
        &mut self,
        mov: MrXMove,
        // TODO: maybe don't pass in the whole state...
        detective_locations: &[DetectiveState],
    ) -> Result<(), MrXMoveError> {
        let new_remaining_tickets = self
            .remaining_tickets
            .use_tickets(mov.first.ticket, mov.second.map(|mov| mov.ticket))?;

        let current_station = self.current_station();

        Self::validate_single_move(current_station, mov.first, detective_locations)?;

        if let Some(additional_move) = mov.second {
            // Double move!

            Self::validate_single_move(
                mov.first.destination,
                additional_move,
                detective_locations,
            )?;
        }

        // Update state.
        self.remaining_tickets = new_remaining_tickets;
        self.moves.push(mov);

        Ok(())
    }
}
