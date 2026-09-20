use std::fmt::Display;

use scotland_yard_common::{DetectiveTicket, MrXTicket, Station, connections::Connection, content};

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
    pub remaining_tickets: RemainingDetectiveTickets,
    pub moves: Vec<DetectiveMove>,
}

impl DetectiveState {
    #[must_use]
    pub const fn new(start: Station) -> Self {
        Self {
            start,
            remaining_tickets: RemainingDetectiveTickets::new(),
            moves: Vec::new(),
        }
    }

    #[must_use]
    pub fn current_station(&self) -> Station {
        self.moves.last().map_or(self.start, |m| m.destination)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct RemainingDetectiveTickets {
    taxi: u8,
    bus: u8,
    underground: u8,
}

impl Default for RemainingDetectiveTickets {
    fn default() -> Self {
        Self::new()
    }
}

impl RemainingDetectiveTickets {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            taxi: content::DETECTIVE_INITIAL_TAXI_TICKET_COUNT,
            bus: content::DETECTIVE_INITIAL_BUS_TICKET_COUNT,
            underground: content::DETECTIVE_INITIAL_UNDERGROUND_TICKET_COUNT,
        }
    }

    /// # Errors
    ///
    /// TODO
    pub fn use_ticket(self, ticket: DetectiveTicket) -> Result<Self, DetectiveMoveError> {
        macro_rules! use_that_ticket {
            ($field:ident) => {{
                if let Some(new_count) = self.$field.checked_sub(1) {
                    Ok(Self {
                        $field: new_count,
                        ..self
                    })
                } else {
                    Err(DetectiveMoveError::NoTicketsLeftOf(ticket))
                }
            }};
        }

        match ticket {
            DetectiveTicket::Taxi => use_that_ticket!(taxi),
            DetectiveTicket::Bus => use_that_ticket!(bus),
            DetectiveTicket::Underground => use_that_ticket!(underground),
        }
    }
}
