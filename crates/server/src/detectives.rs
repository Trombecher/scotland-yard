use scotland_yard_common::{
    MrXTicket, Station, Ticket,
    connections::{CONNECTIONS, Connection},
    content,
};

#[derive(Debug, thiserror::Error)]
pub enum DetectiveMoveError {
    #[error("cannot move detective with {ticket} to {to}", ticket = .0.ticket, to = .0.destination)]
    CannotMove(DetectiveMove),
}

/// This move is not guaranteed to be valid.
#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub struct DetectiveMove {
    pub ticket: Ticket,
    pub destination: Station,
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
    pub fn move_to(&mut self, detective_move: DetectiveMove) -> Result<(), DetectiveMoveError> {
        let current_station = self.current_station();

        if !CONNECTIONS.has(detective_move.connection_from_station(current_station)) {
            self.moves.push(detective_move);

            return Err(DetectiveMoveError::CannotMove(detective_move));
        }

        Ok(())
    }
}
