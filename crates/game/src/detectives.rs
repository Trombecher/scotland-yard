use std::fmt::Display;

use scotland_yard_common::{
    DetectiveTicket, MrXTicket, Station,
    connections::{CONNECTIONS, Connection},
    content,
};

#[derive(Debug, thiserror::Error)]
pub enum DetectiveMoveError {
    #[error("connection does not exist")]
    ConnectionDoesNotExist,
    #[error("no {0} tickets left")]
    NoTicketsLeftOf(DetectiveTicket),
    #[error("detective {detective_at_destination} is already at the destination")]
    ThereIsAlreadyADetectiveAtTheDestination { detective_at_destination: u8 },
}

/// This move is not guaranteed to be valid.
#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub struct DetectiveMove {
    pub ticket: DetectiveTicket,
    pub destination: Station,
}

impl Display for DetectiveMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "with {} to {}", self.ticket, self.destination)
    }
}

impl DetectiveMove {
    #[must_use]
    pub fn connection_from_station(self, from: Station) -> Connection {
        Connection {
            from,
            kind: MrXTicket::from(self.ticket).into(),
            to: self.destination,
        }
    }
}

pub struct DetectiveState {
    pub start: Station,
    pub remaining_taxi_tickets: u8,
    pub remaining_bus_tickets: u8,
    pub remaining_underground_tickets: u8,
    pub moves: Vec<DetectiveMove>,
}

impl DetectiveState {
    #[must_use]
    pub const fn new(start: Station) -> Self {
        Self {
            start,
            remaining_taxi_tickets: content::DETECTIVE_INITIAL_TAXI_TICKET_COUNT,
            remaining_bus_tickets: content::DETECTIVE_INITIAL_BUS_TICKET_COUNT,
            remaining_underground_tickets: content::DETECTIVE_INITIAL_UNDERGROUND_TICKET_COUNT,
            moves: Vec::new(),
        }
    }

    #[must_use]
    pub fn current_station(&self) -> Station {
        self.moves.last().map_or(self.start, |m| m.destination)
    }

    /// Tries to apply the given move.
    ///
    /// # Errors
    ///
    /// If invariants are dissatisfied.
    pub fn move_to(&mut self, detective_move: DetectiveMove) -> Result<(), DetectiveMoveError> {
        let current_station = self.current_station();

        if !CONNECTIONS.has(detective_move.connection_from_station(current_station)) {
            return Err(DetectiveMoveError::ConnectionDoesNotExist);
        }

        // Handle tickets
        match detective_move.ticket {
            DetectiveTicket::Taxi => {
                if let Some(new_count) = self.remaining_taxi_tickets.checked_sub(1) {
                    self.remaining_taxi_tickets = new_count;
                } else {
                    return Err(DetectiveMoveError::NoTicketsLeftOf(detective_move.ticket));
                }
            }
            DetectiveTicket::Bus => todo!(),
            DetectiveTicket::Underground => todo!(),
        }

        self.moves.push(detective_move);

        Ok(())
    }
}
