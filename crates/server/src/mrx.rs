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
}
