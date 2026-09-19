use scotland_yard_common::{Field, Ticket, is_connection};

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub struct DetectiveMove {
    ticket: Ticket,
    destination: Field,
}

impl DetectiveMove {
    #[must_use]
    pub fn new(from: Field, with: Ticket, to: Field) -> Option<Self> {
        is_connection(from, with.into(), to).then_some(Self {
            ticket: with,
            destination: to,
        })
    }
}

pub struct Detective {
    start: Field,
    remaining_taxi_tickets: u8,
    remaining_bus_tickets: u8,
    remaining_underground_tickets: u8,
    moves: Vec<DetectiveMove>,
}

impl Detective {
    #[must_use]
    pub fn current_field(&self) -> Field {
        self.moves.last().map_or(self.start, |m| m.destination)
    }

    pub fn try_use_ticket_to(&mut self, with: Ticket, to: Field) -> bool {
        let mov = DetectiveMove::new(self.current_field(), with, to);
        mov.inspect(|mov| {
            self.moves.push(*mov);
        });
        mov.is_some()
    }
}
