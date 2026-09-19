use std::range::Range;

use scotland_yard_common::{
    Station,
    content::{MAX_DETECTIVES, MIN_DETECTIVES},
};

use crate::{
    detectives::{DetectiveMove, DetectiveMoveError, DetectiveState},
    mrx::{MrXMove, MrXMoveError, MrXState},
};

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
    #[error("error while moving detective {detective_index}: {error}")]
    DetectiveMoveError {
        detective_index: u8,
        error: DetectiveMoveError,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Turn {
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
    mrx: MrXState,
    detectives: Vec<DetectiveState>,
}

impl GameState {
    pub fn new(
        mr_x_starting_station: Station,
        detective_starting_stations: &[Station],
    ) -> Option<Self> {
        if (Range {
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
            detectives: detective_starting_stations
                .iter()
                .copied()
                .map(DetectiveState::new)
                .collect(),
            mrx: MrXState::new(
                mr_x_starting_station,
                detective_starting_stations.len() as u8,
            )
            .unwrap(),
        })
    }

    pub fn current_turn(&self) -> &Turn {
        &self.next_turn
    }

    pub fn mrx(&self) -> &MrXState {
        &self.mrx
    }

    pub fn detectives(&self) -> &[DetectiveState] {
        &self.detectives
    }

    pub fn advance_turn(&mut self) {
        match self.next_turn {
            Turn::MrX => {
                self.next_turn = Turn::Detective(0);
            }
            Turn::Detective(detective_index) => {
                if detective_index as usize + 1 < self.detectives.len() {
                    self.next_turn = Turn::Detective(detective_index + 1);
                } else {
                    self.next_turn = Turn::MrX;
                }
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        // Game is over if MrX is caught or if MrX has no moves left
        self.mrx.is_caught(&self.detectives) || self.mrx.has_no_moves()
    }

    pub fn can_move_mr_x(&self) -> bool {
        self.next_turn == Turn::MrX && !self.is_game_over()
    }

    pub fn can_move_detective(&self, detective_index: u8) -> bool {
        match self.next_turn {
            Turn::Detective(index) => index == detective_index && !self.is_game_over(),
            _ => false,
        }
    }

    pub fn move_mr_x(&mut self, mov: MrXMove) -> Result<(), GameStateTransitionError> {
        if !self.can_move_mr_x() {
            return Err(GameStateTransitionError::ItIsNotTheTurnOfMrX);
        }

        self.mrx.move_by(mov, &self.detectives)?;
        self.advance_turn();
        Ok(())
    }

    fn validate_detective_index(
        &self,
        detective_index: u8,
    ) -> Result<(), InvalidDetectiveIndexError> {
        if detective_index as usize >= self.detectives.len() {
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
        Ok(self.detectives.get(detective_index as usize).unwrap())
    }

    /// This is not exposed due to potential state manipulation.
    fn detective_mut(
        &mut self,
        detective_index: u8,
    ) -> Result<&mut DetectiveState, InvalidDetectiveIndexError> {
        self.validate_detective_index(detective_index)?;

        // this unwrap won't panic.
        Ok(self.detectives.get_mut(detective_index as usize).unwrap())
    }

    pub fn move_detective(
        &mut self,
        detective_index: u8,
        mov: DetectiveMove,
    ) -> Result<(), GameStateTransitionError> {
        if !self.can_move_detective(detective_index) {
            return Err(GameStateTransitionError::IsIsNotTheTurnOfDetective(
                detective_index,
            ));
        }

        self.detective_mut(detective_index)
            .map_err(GameStateTransitionError::from)?
            .move_to(mov)
            .map_err(|error| GameStateTransitionError::DetectiveMoveError {
                detective_index,
                error,
            })?;

        self.advance_turn();

        Ok(())
    }
}
