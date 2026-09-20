use scotland_yard_common::{
    MrXTicket, Station,
    connections::{CONNECTIONS, ConnectionKind},
    content::{self, MAX_DETECTIVES, MIN_DETECTIVES},
};

use crate::{
    MrXMove, MrXMoveError, StoredMrXMove,
    detectives::{DetectiveMove, DetectiveMoveError, DetectiveState},
    mr_x::MrXState,
};

#[derive(Copy, Clone, Debug)]
pub enum GameState {
    MrXHasWon,
    DetectivesHaveWon {
        detective_index_which_captured_mr_x: u8,
    },
    PendingTurn {
        turn: Turn,
        round: u8,
    },
    Draw,
}

#[derive(Debug, thiserror::Error)]
#[error("{0} is not a valid index for a detective")]
pub struct InvalidDetectiveIndexError(pub u8);

#[derive(Debug, thiserror::Error)]
pub enum GameStateTransitionError {
    #[error("Mr. X has already won")]
    MrXHasAlreadyWon,
    #[error("detectives have already won")]
    DetectivesHaveAlreadyWon,
    #[error("it is not the turn of Mr. X")]
    ItIsNotTheTurnOfMrX,
    #[error("{0}")]
    InvalidDetectiveIndex(#[from] InvalidDetectiveIndexError),
    #[error("game has already ended in a draw")]
    GameIsAlreadyDrawed,
    #[error("it is not the turn of detective #{0}")]
    IsIsNotTheTurnOfDetective(u8),
    #[error("error while moving Mr. X: {0}")]
    MrXMoveError(#[from] MrXMoveError),
    #[error("error trying to move detective {detective_index} {detective_move}: {error}")]
    DetectiveMoveError {
        detective_index: u8,
        detective_move: DetectiveMove,
        error: DetectiveMoveError,
    },
}

/// Indicates who's turn it is.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Turn {
    Detective(u8),
    MrX {
        /// Indicates whether it is Mr. X's turn
        /// because he used a double ticket, or
        /// not.
        is_double: bool,
    },
}

impl Turn {
    #[must_use]
    pub fn advance(
        self,
        round: u8,
        detective_count: u8,
        mr_x_used_double_ticket: bool,
    ) -> (Self, u8) {
        match self {
            Self::Detective(detective_index) if detective_index == detective_count + 1 => {
                (Self::MrX { is_double: false }, round + 1)
            }
            Self::Detective(detective_index) => (Self::Detective(detective_index + 1), round),
            Self::MrX { is_double: false } if mr_x_used_double_ticket => {
                (Self::MrX { is_double: true }, round + 1)
            }
            Self::MrX { is_double: _ } => (Self::Detective(0), round),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DetectiveCount {
    One,
    Two,
    Three,
    Four,
}

impl TryFrom<u8> for DetectiveCount {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            _ => Err(()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameStateCreationError {
    #[error("{0} is too few detectives")]
    TooFewDetectives(u8),

    #[error("{0} is too many detectives")]
    TooManyDetectives(usize),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameMove {
    MrX {
        mov: MrXMove,
        use_double_ticket: bool,
    },
    Detective {
        index: u8,
        mov: DetectiveMove,
    },
}

pub struct Game {
    round: u8,
    next_turn: Turn,
    mr_x_state: MrXState,
    detective_states: Vec<DetectiveState>,
}

impl Game {
    /// # Errors
    ///
    /// TODO
    #[allow(clippy::missing_panics_doc)]
    pub fn new(
        mr_x_starting_station: Station,
        detective_starting_stations: &[Station],
    ) -> Result<Self, GameStateCreationError> {
        if detective_starting_stations.len() < MIN_DETECTIVES as usize {
            // This cast does not truncate.
            #[allow(clippy::cast_possible_truncation)]
            return Err(GameStateCreationError::TooFewDetectives(
                detective_starting_stations.len() as u8,
            ));
        }

        if detective_starting_stations.len() > MAX_DETECTIVES as usize {
            return Err(GameStateCreationError::TooManyDetectives(
                detective_starting_stations.len(),
            ));
        }

        Ok(Game {
            next_turn: Turn::MrX { is_double: false },
            round: 0,
            detective_states: detective_starting_stations
                .iter()
                .copied()
                .map(DetectiveState::new)
                .collect(),

            #[allow(clippy::cast_possible_truncation)]
            mr_x_state: MrXState::new(
                mr_x_starting_station,
                detective_starting_stations.len() as u8,
            )
            .unwrap(),
        })
    }

    #[must_use]
    pub fn mr_x(&self) -> &MrXState {
        &self.mr_x_state
    }

    #[must_use]
    pub fn detectives(&self) -> &[DetectiveState] {
        &self.detective_states
    }

    /// Returns the index of the detective standing on Mr. X's
    /// station; or `None`, iff there is none.
    fn detective_standing_on_mr_x_station(&self) -> Option<u8> {
        let mr_x_station = self.mr_x_state.current_station();

        self.detective_states
            .iter()
            .enumerate()
            .find_map(|(detective_index, detective)| {
                (detective.current_station() == mr_x_station).then_some(detective_index as u8)
            })
    }

    #[must_use]
    pub fn state(&self) -> GameState {
        if let Some(detective_index) = self.detective_standing_on_mr_x_station() {
            GameState::DetectivesHaveWon {
                detective_index_which_captured_mr_x: detective_index,
            }
        } else if !self.is_a_detective_able_to_move() || self.round >= content::ROUNDS {
            GameState::MrXHasWon
        } else if !self.is_mr_x_able_to_move() {
            GameState::Draw
        } else {
            GameState::PendingTurn {
                turn: self.next_turn,
                round: self.round,
            }
        }
    }

    fn validate_detective_index(
        &self,
        detective_index: u8,
    ) -> Result<(), InvalidDetectiveIndexError> {
        if detective_index as usize >= self.detective_states.len() {
            return Err(InvalidDetectiveIndexError(detective_index));
        }

        Ok(())
    }

    /// # Errors
    ///
    /// If the detective with the given index does not exist.
    #[allow(clippy::missing_panics_doc)]
    pub fn detective(
        &self,
        detective_index: u8,
    ) -> Result<&DetectiveState, InvalidDetectiveIndexError> {
        self.validate_detective_index(detective_index)?;

        // this unwrap won't panic.
        Ok(self.detective_states.get(detective_index as usize).unwrap())
    }

    /// This is not exposed due to potential state manipulation.
    fn detective_mut(
        &mut self,
        detective_index: u8,
    ) -> Result<&mut DetectiveState, InvalidDetectiveIndexError> {
        self.validate_detective_index(detective_index)?;

        // this unwrap won't panic.
        Ok(self
            .detective_states
            .get_mut(detective_index as usize)
            .unwrap())
    }

    fn detectives_at(&self, station: Station) -> impl Iterator<Item = u8> {
        self.detective_states
            .iter()
            .enumerate()
            .filter_map(move |(index, detective)| {
                (detective.current_station() == station).then_some(index)
            })
            .map(|detective_index| {
                #[allow(clippy::cast_possible_truncation)]
                return detective_index as u8;
            })
    }

    #[must_use]
    fn is_a_detective_able_to_move(&self) -> bool {
        (0..self.detective_states.len() as u8)
            .any(|detective_index| self.is_detective_able_to_move(detective_index).unwrap())
    }

    fn is_detective_able_to_move(
        &self,
        detective_index: u8,
    ) -> Result<bool, InvalidDetectiveIndexError> {
        let detective = self.detective(detective_index)?;
        let detective_station = detective.current_station();

        Ok(detective
            .remaining_tickets
            .available_tickets()
            .map(MrXTicket::from)
            .map(ConnectionKind::from)
            .any(|connection_kind| {
                CONNECTIONS
                    .destinations(detective_station, connection_kind)
                    .any(|destination| self.detectives_at(destination).next().is_none())
            }))
    }

    fn is_mr_x_able_to_move(&self) -> bool {
        let mr_x_station = self.mr_x_state.current_station();

        if self
            .mr_x_state
            .remaining_tickets
            .available_tickets()
            .map(ConnectionKind::from)
            .any(|connection_kind| {
                CONNECTIONS
                    .destinations(mr_x_station, connection_kind)
                    .any(|destination| self.detectives_at(destination).next().is_none())
            })
        {
            // Mr. X has a ticket and a station
            // such that the current Mr. X station connects
            // and there does not exist a detective at that
            // destination station.
            //
            // Therefore, Mr. X can move there.

            return true;
        }

        false
    }

    fn validate_mr_x_single_move(
        &self,
        from: Station,
        mr_x_single_move: MrXMove,
    ) -> Result<(), MrXMoveError> {
        // Check that this is a valid connection.
        if !CONNECTIONS.has(mr_x_single_move.connection_from_station(from)) {
            return Err(MrXMoveError::ConnectionDoesNotExist {
                from,
                mov: mr_x_single_move,
            });
        }

        // Check that there is no detective at the destination.
        if let Some(detective_index) =
            self.detective_states
                .iter()
                .enumerate()
                .find_map(|(detective_index, state)| {
                    (state.current_station() == mr_x_single_move.destination)
                        .then_some(detective_index)
                })
        {
            return Err(MrXMoveError::CannotMoveToStationBecauseDetectiveIsThere {
                detective_index,
                single_move: mr_x_single_move,
            });
        }

        Ok(())
    }

    /// Transition into a new game state via a move.
    ///
    /// # Errors
    ///
    /// If the move is invalid.
    pub fn transition(&mut self, mov: GameMove) -> Result<(), GameStateTransitionError> {
        let (turn, round) = match self.state() {
            GameState::MrXHasWon => return Err(GameStateTransitionError::MrXHasAlreadyWon),
            GameState::DetectivesHaveWon { .. } => {
                return Err(GameStateTransitionError::DetectivesHaveAlreadyWon);
            }
            GameState::PendingTurn { turn, round } => (turn, round),
            GameState::Draw => return Err(GameStateTransitionError::GameIsAlreadyDrawed),
        };

        // Do state modification.
        let mr_x_uses_double_ticket = match mov {
            GameMove::MrX {
                mov: mr_x_move,
                use_double_ticket,
            } => {
                let Turn::MrX {
                    is_double: this_is_the_double_move,
                } = turn
                else {
                    return Err(GameStateTransitionError::ItIsNotTheTurnOfMrX);
                };

                if this_is_the_double_move && use_double_ticket {
                    return Err(GameStateTransitionError::MrXMoveError(
                        MrXMoveError::CannotUseDoubleTicketTwice,
                    ));
                }

                let new_remaining_mr_x_tickets = if use_double_ticket {
                    self.mr_x_state
                        .remaining_tickets
                        .use_ticket(mr_x_move.ticket)
                        .map_err(MrXMoveError::from)?
                } else {
                    self.mr_x_state
                        .remaining_tickets
                        .use_ticket(mr_x_move.ticket)
                        .map_err(MrXMoveError::from)?
                        .use_double_move_ticket()?
                };

                let current_station = self.mr_x_state.current_station();

                self.validate_mr_x_single_move(current_station, mr_x_move)?;

                // Update state.
                self.mr_x_state.remaining_tickets = new_remaining_mr_x_tickets;
                self.mr_x_state.moves.push(StoredMrXMove {
                    mov: mr_x_move,
                    used_double_ticket: use_double_ticket,
                });

                use_double_ticket
            }
            GameMove::Detective {
                index: detective_index,
                mov: detective_move,
            } => {
                if !matches!(turn, Turn::Detective(n) if n == detective_index) {
                    return Err(GameStateTransitionError::IsIsNotTheTurnOfDetective(
                        detective_index,
                    ));
                }

                // Check if there is already another detective at the destination
                if let Some(detective_at_destination) =
                    self.detectives_at(detective_move.destination).next()
                {
                    return Err(GameStateTransitionError::DetectiveMoveError {
                        detective_index,
                        detective_move,
                        error: DetectiveMoveError::ThereIsAlreadyADetectiveAtTheDestination {
                            detective_at_destination,
                        },
                    });
                }

                let detective = self.detective_mut(detective_index)?;
                let detective_current_station = detective.current_station();

                if !CONNECTIONS
                    .has(detective_move.connection_from_station(detective_current_station))
                {
                    return Err(GameStateTransitionError::DetectiveMoveError {
                        detective_index,
                        detective_move,
                        error: DetectiveMoveError::ConnectionDoesNotExist,
                    });
                }

                // Update detective state.
                detective
                    .remaining_tickets
                    .use_ticket(detective_move.ticket)
                    .map_err(|error| GameStateTransitionError::DetectiveMoveError {
                        detective_index,
                        error,
                        detective_move,
                    })?;

                detective.moves.push(detective_move);

                false
            }
        };

        // Advance turn.
        (self.next_turn, self.round) = turn.advance(
            round,
            self.detective_states.len() as u8,
            mr_x_uses_double_ticket,
        );

        Ok(())
    }
}
