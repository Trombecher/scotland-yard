use scotland_yard_common::{Field, MrXTicket};

pub struct SingleMrXMove {
    ticket: MrXTicket,
    destination: Field,
}

pub enum MrXMove {
    Single(SingleMrXMove),
    Double(SingleMrXMove, SingleMrXMove),
}

pub struct MrX {
    start: Field,
    moves: Vec<MrXMove>,
}

impl MrX {
    #[must_use]
    pub const fn new(start: Field) -> Self {
        Self {
            start,
            moves: Vec::new(),
        }
    }

    #[must_use]
    pub fn current_field(&self) -> Field {
        self.moves.last().map_or(self.start, |mov| match mov {
            MrXMove::Single(m) => m.destination,
            MrXMove::Double(_, m) => m.destination,
        })
    }

    pub fn move_by(&mut self, tickets: &[MrXTicket]) -> Result<(), String> {
        if tickets.is_empty() {
            return Err("At least one ticket must be provided".to_string());
        }

        if tickets.len() > 2 {
            return Err("MrX can only use 1 or 2 tickets per move".to_string());
        }

        // Create the move based on number of tickets
        let move_result = match tickets.len() {
            1 => {
                let ticket = tickets[0];
                // In a real implementation, we would validate that this ticket is valid
                // for the current position and return the next field based on the game rules
                // For now, we'll just create a dummy implementation
                let destination = Field::new(0); // This would need to be properly implemented
                MrXMove::Single(SingleMrXMove {
                    ticket,
                    destination,
                })
            }
            2 => {
                let ticket1 = tickets[0];
                let ticket2 = tickets[1];
                // In a real implementation, we would validate both tickets and return the next field
                // For now, we'll just create a dummy implementation
                let destination = Field::new(0); // This would need to be properly implemented
                MrXMove::Double(
                    SingleMrXMove {
                        ticket: ticket1,
                        destination,
                    },
                    SingleMrXMove {
                        ticket: ticket2,
                        destination,
                    },
                )
            }
            _ => return Err("Invalid number of tickets".to_string()),
        };

        self.moves.push(move_result);
        Ok(())
    }

    pub fn is_caught(&self, detectives: &[crate::detectives::Detective]) -> bool {
        let current_field = self.current_field();
        detectives
            .iter()
            .any(|detective| detective.current_field() == current_field)
    }

    pub fn has_no_moves(&self) -> bool {
        // In a real implementation, this would check if MrX has any valid moves left
        // For now, we'll just return false as an initial placeholder
        false
    }
}
