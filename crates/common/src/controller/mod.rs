mod random;

pub use random::*;

use crate::board::Field;
use crate::game::{AdditionalSeekerInfo, MrXMove, MrXOpaqueMove, Ticket};

pub trait MrXController {
    fn init(&mut self, starting_field: Field) {}

    fn next_move(
        &mut self,
        seeker_positions: &[Field],
        additional_seeker_info: Option<&[AdditionalSeekerInfo]>,
    ) -> MrXMove;
}

pub trait SeekerController {
    fn init(&mut self, starting_field: Field) {}
    
    fn next_message(&mut self) -> Option<&str> {
        None
    }

    fn next_move(
        &mut self,
        last_revealed_mr_x_position: Field,
        last_mr_x_move: MrXOpaqueMove,
    ) -> Option<(Ticket, Field)>;
}
