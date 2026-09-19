use std::fmt::Display;

use scotland_yard_common::{
    MrXTicket, Station,
    connections::{CONNECTIONS, Connection},
    content::DETECTIVE_COUNT_RANGE,
};

use crate::detectives::DetectiveState;

#[derive(Debug, thiserror::Error)]
pub enum MrXMoveError {
    #[error("the connection {from} {mov} does not exist")]
    ConnectionDoesNotExist { from: Station, mov: SingleMrXMove },
    #[error("cannot double move {0} because Mr. X has no more double move tickets")]
    CannotDoubleMoveDueToMissingDoubleMoveTicket(SingleMrXMove),
    #[error(
        "cannot move Mr. X {single_move} because detective #{detective_index} is already there"
    )]
    CannotMoveToStationBecauseDetectiveIsThere {
        detective_index: usize,
        single_move: SingleMrXMove,
    },
}

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
    double_move_tickets_available: u8,
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
            double_move_tickets_available: 2,
        })
    }

    #[must_use]
    pub fn current_station(&self) -> Station {
        self.moves
            .last()
            .copied()
            .map_or(self.start, MrXMove::destination)
    }

    fn validate_single_move(
        from: Station,
        mov: SingleMrXMove,
        detective_locations: &[DetectiveState],
    ) -> Result<(), MrXMoveError> {
        // TODO: check ticket availability.

        // Check that this is a valid connection.
        if !CONNECTIONS.has(mov.connection_from_station(from)) {
            return Err(MrXMoveError::ConnectionDoesNotExist { from, mov });
        }

        // Check that there is no detective at the destination.
        if let Some(detective_index) =
            detective_locations
                .iter()
                .enumerate()
                .find_map(|(detective_index, state)| {
                    (state.current_station() == mov.destination).then_some(detective_index)
                })
        {
            return Err(MrXMoveError::CannotMoveToStationBecauseDetectiveIsThere {
                detective_index,
                single_move: mov,
            });
        }

        Ok(())
    }

    /// Tries to move Mr. X.
    ///
    /// # Errors
    ///
    /// If invariants are dissatisfied.
    pub fn move_by(
        &mut self,
        mov: MrXMove,
        // TODO: maybe don't pass in the whole state...
        detective_locations: &[DetectiveState],
    ) -> Result<(), MrXMoveError> {
        let current_station = self.current_station();

        Self::validate_single_move(current_station, mov.first, detective_locations)?;

        if let Some(additional_move) = mov.second {
            // Double move! Validate, that Mr. X has at least one double move ticket.

            if self.double_move_tickets_available == 0 {
                return Err(MrXMoveError::CannotDoubleMoveDueToMissingDoubleMoveTicket(
                    additional_move,
                ));
            }

            Self::validate_single_move(
                mov.first.destination,
                additional_move,
                detective_locations,
            )?;

            self.double_move_tickets_available -= 1;
        }

        self.moves.push(mov);

        Ok(())
    }
}
