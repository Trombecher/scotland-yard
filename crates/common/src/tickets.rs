use crate::ConnectionType;

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

impl From<Ticket> for ConnectionType {
    fn from(value: Ticket) -> Self {
        match value {
            Ticket::Taxi => Self::Taxi,
            Ticket::Bus => Self::Bus,
            Ticket::Underground => Self::Underground,
        }
    }
}
