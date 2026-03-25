use scotland_yard_common::Ticket;

use crate::{detectives::Detective, mrx::MrX};

pub struct GameState {
    mrx: MrX,
    detectives: Vec<Detective>,
}
