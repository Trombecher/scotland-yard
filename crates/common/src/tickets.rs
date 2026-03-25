#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MrXTicket {
    Taxi,
    Bus,
    Metro,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Ticket {
    Taxi,
    Bus,
    Metro,
}
