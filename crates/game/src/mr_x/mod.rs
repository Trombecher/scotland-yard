mod errors;
mod tickets;

pub use errors::*;
pub use tickets::*;

use std::fmt::Display;

use scotland_yard_common::{
    MrXTicket, Station, connections::Connection, content::DETECTIVE_COUNT_RANGE,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MrXMove {
    pub ticket: MrXTicket,
    pub destination: Station,
}

impl Display for MrXMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "with {} to {}", self.ticket, self.destination)
    }
}

impl MrXMove {
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
pub struct StoredMrXMove {
    pub mov: MrXMove,
    pub used_double_ticket: bool,
}

pub struct MrXState {
    pub start: Station,
    pub moves: Vec<StoredMrXMove>,
    pub remaining_tickets: RemainingMrXTickets,
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

    #[must_use]
    pub fn current_station(&self) -> Station {
        self.moves
            .last()
            .copied()
            .map_or(self.start, |stored_move| stored_move.mov.destination)
    }
}
