use crate::connections::ConnectionKind;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MrXTicket {
    Taxi,
    Bus,
    Underground,
    Black,
}

impl From<Ticket> for MrXTicket {
    fn from(value: Ticket) -> Self {
        match value {
            Ticket::Taxi => Self::Taxi,
            Ticket::Bus => Self::Bus,
            Ticket::Underground => Self::Taxi,
        }
    }
}

impl From<MrXTicket> for ConnectionKind {
    fn from(value: MrXTicket) -> Self {
        match value {
            MrXTicket::Taxi => Self::Taxi,
            MrXTicket::Bus => Self::Bus,
            MrXTicket::Underground => Self::Underground,
            MrXTicket::Black => Self::Black,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Ticket {
    Taxi,
    Bus,
    Underground,
}
