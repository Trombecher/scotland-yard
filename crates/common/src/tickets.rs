use std::fmt::{Display, Write};

use crate::connections::ConnectionKind;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MrXTicket {
    Taxi,
    Bus,
    Underground,
    Black,
}

impl Display for MrXTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Taxi => f.write_str("a taxi ticket"),
            Self::Bus => f.write_str("a bus ticket"),
            Self::Underground => f.write_str("an underground ticket"),
            Self::Black => f.write_str("a black ticket"),
        }
    }
}

impl From<Ticket> for MrXTicket {
    fn from(value: Ticket) -> Self {
        match value {
            Ticket::Taxi => Self::Taxi,
            Ticket::Bus => Self::Bus,
            Ticket::Underground => Self::Underground,
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

impl From<ConnectionKind> for MrXTicket {
    fn from(value: ConnectionKind) -> Self {
        match value {
            ConnectionKind::Taxi => Self::Taxi,
            ConnectionKind::Bus => Self::Bus,
            ConnectionKind::Underground => Self::Underground,
            ConnectionKind::Black => Self::Black,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Ticket {
    Taxi,
    Bus,
    Underground,
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ticket::Taxi => f.write_str("a taxi ticket"),
            Ticket::Bus => f.write_str("a bug ticket"),
            Ticket::Underground => f.write_str("an underground ticket"),
        }
    }
}
