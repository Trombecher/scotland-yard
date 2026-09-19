use std::range::Range;

use scotland_yard_common::{
    Station,
    content::{MAX_DETECTIVES, MIN_DETECTIVES, ROUNDS},
};

use crate::{
    detectives::{DetectiveMove, DetectiveMoveError, DetectiveState},
    mrx::{MrXMove, MrXMoveError, MrXState},
};

#[derive(Clone, Debug)]
pub enum GameStatus {
    MrXHasWon,
    DetectivesHaveWon {
        detective_index_which_captured_mr_x: u8,
    },
    PendingTurn(Turn),
}

impl GameStatus {
    pub fn is_it_the_turn_of_mr_x(self) -> bool {
        matches!(self, Self::PendingTurn(Turn::MrX))
    }

    pub fn is_it_the_turn_of_detective(self, detective_index: u8) -> bool {
        match self {
            Self::PendingTurn(Turn::Detective(d)) => detective_index == d,
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

pub struct GameState {
    next_turn: Turn,
    /// Rounds 0 to 21 (inclusive)
    round: u8,
    mr_x_state: MrXState,
    detective_states: Vec<DetectiveState>,
}

impl GameState {
    pub fn new(
        mr_x_starting_station: Station,
        detective_starting_stations: &[Station],
    ) -> Option<Self> {
        if !(Range {
            start: MIN_DETECTIVES as usize,
            end: MAX_DETECTIVES as usize + 1,
        })
        .contains(&detective_starting_stations.len())
        {
            // Too many or too few detectives
            return None;
        }

        Some(GameState {
            next_turn: Turn::MrX,
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
            round: 0,
        })
    }

    pub fn mr_x(&self) -> &MrXState {
        &self.mr_x_state
    }

    pub fn detectives(&self) -> &[DetectiveState] {
        &self.detective_states
    }

    pub fn round(&self) -> u8 {
        self.round
    }

    /// Advances the turn.
    ///
    /// * If it was Mr. X's turn, then it is now detective 0's term;
    /// * if it was the last detective's turn then it is Mr. X's turn now;
    /// * otherwise, it is now the next detective's turn.
    fn advance_turn(&mut self) {
        match self.next_turn {
            Turn::MrX => {
                self.next_turn = Turn::Detective(0);
            }
            Turn::Detective(detective_index) => {
                if detective_index as usize + 1 < self.detective_states.len() {
                    self.next_turn = Turn::Detective(detective_index + 1);
                } else {
                    self.next_turn = Turn::MrX;
                }
            }
        }
    }

    pub fn status(&self) -> GameStatus {
        if self.round >= ROUNDS {
            GameStatus::MrXHasWon
        } else {
            let mr_x_station = self.mr_x_state.current_station();

            if let Some(detective) = self.detectives_at(mr_x_station).next() {
                GameStatus::DetectivesHaveWon {
                    detective_index_which_captured_mr_x: detective,
                }
            } else {
                todo!()
            }
        }
    }

    pub fn move_mr_x(&mut self, mov: MrXMove) -> Result<(), GameStateTransitionError> {
        if !self.status().is_it_the_turn_of_mr_x() {
            return Err(GameStateTransitionError::ItIsNotTheTurnOfMrX);
        }

        self.mr_x_state.move_by(mov, &self.detective_states)?;
        self.advance_turn();

        Ok(())
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
            .map(|detective_index| detective_index as u8)
    }

    pub fn move_detective(
        &mut self,
        detective_index: u8,
        detective_move: DetectiveMove,
    ) -> Result<(), GameStateTransitionError> {
        if !self.status().is_it_the_turn_of_detective(detective_index) {
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

        self.advance_turn();

        Ok(())
    }
}
