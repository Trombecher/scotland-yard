use scotland_yard_common::Ticket;

use crate::{detectives::Detective, mrx::MrX};

enum Turn {
    Detective(usize),
    MrX,
}

pub struct GameState {
    next_turn: Turn,
    mrx: MrX,
    detectives: Vec<Detective>,
}

impl GameState {
    pub fn new(mrx: MrX, detectives: Vec<Detective>) -> Self {
        GameState {
            next_turn: Turn::MrX,
            mrx,
            detectives,
        }
    }

    pub fn current_turn(&self) -> &Turn {
        &self.next_turn
    }

    pub fn mrx(&self) -> &MrX {
        &self.mrx
    }

    pub fn detectives(&self) -> &[Detective] {
        &self.detectives
    }

    pub fn advance_turn(&mut self) {
        match self.next_turn {
            Turn::MrX => {
                self.next_turn = Turn::Detective(0);
            }
            Turn::Detective(index) => {
                if index + 1 < self.detectives.len() {
                    self.next_turn = Turn::Detective(index + 1);
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

    pub fn can_move_mrx(&self) -> bool {
        self.next_turn == Turn::MrX && !self.is_game_over()
    }

    pub fn can_move_detective(&self, detective_index: usize) -> bool {
        match self.next_turn {
            Turn::Detective(index) => index == detective_index && !self.is_game_over(),
            _ => false,
        }
    }

    pub fn move_mrx(&mut self, tickets: &[Ticket]) -> Result<(), String> {
        if !self.can_move_mrx() {
            return Err("Not MrX's turn".to_string());
        }

        self.mrx.move_by(tickets)?;
        self.advance_turn();
        Ok(())
    }

    pub fn move_detective(
        &mut self,
        detective_index: usize,
        tickets: &[Ticket],
    ) -> Result<(), String> {
        if !self.can_move_detective(detective_index) {
            return Err("Not detective's turn".to_string());
        }

        if detective_index >= self.detectives.len() {
            return Err("Invalid detective index".to_string());
        }

        self.detectives[detective_index].move_by(tickets)?;
        self.advance_turn();
        Ok(())
    }
}
