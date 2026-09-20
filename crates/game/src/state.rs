use scotland_yard_common::{
    Station,
    content::{MAX_DETECTIVES, MIN_DETECTIVES, ROUNDS},
};

use crate::{
    MrXMoveError,
    detectives::{DetectiveMove, DetectiveMoveError, DetectiveState},
    mr_x::{MrXMove, MrXState},
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
}

impl GameState {
    #[must_use]
    pub fn is_it_the_turn_of_mr_x(self) -> bool {
        matches!(
            self,
            Self::PendingTurn {
                turn: Turn::MrX,
                ..
            }
        )
    }

    #[must_use]
    pub fn is_it_the_turn_of_detective(self, detective_index: u8) -> bool {
        match self {
            Self::PendingTurn {
                turn: Turn::Detective(d),
                ..
            } => detective_index == d,
            _ => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0} is not a valid index for a detective")]
pub struct InvalidDetectiveIndexError(pub u8);

#[derive(Debug, thiserror::Error)]
pub enum GameStateTransitionError {
    #[error("it is not the turn of Mr. X")]
    ItIsNotTheTurnOfMrX,
    #[error("{0}")]
    InvalidDetectiveIndex(#[from] InvalidDetectiveIndexError),
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
    MrX,
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
    MrX(MrXMove),
    Detective { index: u8, mov: DetectiveMove },
}

pub struct Game {
    state: GameState,
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
            state: GameState::PendingTurn {
                turn: Turn::MrX,
                round: 0,
            },

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

    #[must_use]
    pub fn state(&self) -> GameState {
        self.state
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

    /// Transition into a new game state via a move.
    ///
    /// # Errors
    ///
    /// If the move is invalid.
    pub fn transition(&mut self, mov: GameMove) -> Result<(), GameStateTransitionError> {
        match mov {
            GameMove::MrX(mr_x_move) => {
                if !self.state().is_it_the_turn_of_mr_x() {
                    return Err(GameStateTransitionError::ItIsNotTheTurnOfMrX);
                }

                self.mr_x_state.move_by(mr_x_move, &self.detective_states)?;
            }
            GameMove::Detective {
                index: detective_index,
                mov: detective_move,
            } => {
                // We try to move the detective.

                if !self.state().is_it_the_turn_of_detective(detective_index) {
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

                self.detective_mut(detective_index)
                    .map_err(GameStateTransitionError::from)?
                    .move_to(detective_move)
                    .map_err(|error| GameStateTransitionError::DetectiveMoveError {
                        detective_index,
                        error,
                        detective_move,
                    })?;

                self.transition_game_state();
            }
        }

        self.transition_game_state();

        Ok(())
    }

    /// Transitions the game into a new state.
    ///
    /// * If it was Mr. X's turn, then it is now detective 0's term;
    /// * if it was the last detective's turn then it is Mr. X's turn now;
    /// * otherwise, it is now the next detective's turn.
    fn transition_game_state(&mut self) {
        let (turn, round) = match self.state {
            GameState::MrXHasWon => {
                unreachable!("advanced turn but Mr. X has already won")
            }
            GameState::DetectivesHaveWon {
                detective_index_which_captured_mr_x,
            } => {
                unreachable!(
                    "advanced turn but detective {detective_index_which_captured_mr_x} have already won"
                )
            }
            GameState::PendingTurn { turn, round } => (turn, round),
        };

        let mr_x_station = self.mr_x_state.current_station();
        let detective_at_mr_x_station = self.detectives_at(mr_x_station).next();

        if let Some(detective) = detective_at_mr_x_station {
            self.state = GameState::DetectivesHaveWon {
                detective_index_which_captured_mr_x: detective,
            };

            return;
        }

        self.state = match turn {
            Turn::MrX => GameState::PendingTurn {
                turn: Turn::Detective(0),
                round,
            },
            Turn::Detective(detective_index)
                if self.detective_states.len() == detective_index as usize + 1 =>
            {
                // It was last detective's turn and they have not won.

                if round == ROUNDS - 1 {
                    // It is last round.

                    GameState::MrXHasWon
                } else {
                    // Not last round.

                    GameState::PendingTurn {
                        turn: Turn::MrX,
                        round: round + 1,
                    }
                }
            }
            Turn::Detective(detective_index) => GameState::PendingTurn {
                turn: Turn::Detective(detective_index + 1),
                round,
            },
        };
    }
}
