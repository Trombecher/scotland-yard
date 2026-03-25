#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MrXTicket {
    Taxi,
    Bus,
    Underground,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Ticket {
    Taxi,
    Bus,
    Underground,
}
