use scotland_yard_common::{
    MrXTicket, Station,
    connections::{CONNECTIONS, Connection},
    content::DETECTIVE_COUNT_RANGE,
};

use crate::detectives::DetectiveState;

#[derive(Debug, thiserror::Error)]
pub enum MrXMoveError {
    #[error("cannot move Mr. X with {ticket} to {to}", ticket = .0.ticket, to = .0.destination)]
    CannotMove(SingleMrXMove),
    #[error(
        "cannot double move Mr. X with {ticket} to {to} because Mr. X has no more double move tickets",
        ticket = .0.ticket, to = .0.destination
    )]
    CannotDoubleMoveDueToMissingDoubleMoveTicket(SingleMrXMove),
    #[error(
        "cannot move Mr. X with {ticket} to {destination} because detective #{detective_index} is already there",
        ticket = single_move.ticket,
        destination = single_move.destination
    )]
    CannotMoveToStationBecauseDetectiveIsThere {
        detective_index: usize,
        single_move: SingleMrXMove,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SingleMrXMove {
    ticket: MrXTicket,
    destination: Station,
}

impl SingleMrXMove {
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
    first: SingleMrXMove,
    second: Option<SingleMrXMove>,
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
        &self,
        from: Station,
        mov: SingleMrXMove,
        detective_locations: &[DetectiveState],
    ) -> Result<(), MrXMoveError> {
        // TODO: check ticket availability.

        // Check that this is a valid connection.
        if !CONNECTIONS.has(mov.connection_from_station(from)) {
            return Err(MrXMoveError::CannotMove(mov));
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

        return Ok(());
    }

    pub fn move_by(
        &mut self,
        mov: MrXMove,
        // TODO: maybe don't pass in the whole state...
        detective_locations: &[DetectiveState],
    ) -> Result<(), MrXMoveError> {
        let current_station = self.current_station();

        self.validate_single_move(current_station, mov.first, detective_locations)?;

        if let Some(additional_move) = mov.second {
            // Double move! Validate, that Mr. X has at least one double move ticket.

            if self.double_move_tickets_available == 0 {
                return Err(MrXMoveError::CannotDoubleMoveDueToMissingDoubleMoveTicket(
                    additional_move,
                ));
            }

            self.validate_single_move(mov.first.destination, additional_move, detective_locations)?;

            self.double_move_tickets_available -= 1;
        }

        self.moves.push(mov);

        Ok(())
    }

    pub fn is_caught(&self, detectives: &[crate::detectives::DetectiveState]) -> bool {
        let current_field = self.current_station();
        detectives
            .iter()
            .any(|detective| detective.current_station() == current_field)
    }

    pub fn has_no_moves(&self) -> bool {
        // In a real implementation, this would check if MrX has any valid moves left
        // For now, we'll just return false as an initial placeholder
        false
    }
}
