use scotland_yard_common::{Field, Ticket};

pub struct DetectiveMove {
    ticket: Ticket,
    destination: Field,
}

pub struct Detective {
    start: Field,
    remaining_taxi_tickets: u8,
    remaining_bus_tickets: u8,
    remaining_underground_tickets: u8,
    moves: Vec<DetectiveMove>,
}

impl Detective {
    pub fn try_use_ticket(&mut self, ticket: Ticket) {}
}
