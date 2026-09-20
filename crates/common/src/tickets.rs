use std::fmt::Display;

use crate::connections::ConnectionKind;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MrXTicket {
    Taxi,
    Bus,
    Underground,
    Black,
}

impl MrXTicket {
    pub const ALL: [Self; 4] = [Self::Taxi, Self::Bus, Self::Underground, Self::Black];
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

impl From<DetectiveTicket> for MrXTicket {
    fn from(value: DetectiveTicket) -> Self {
        match value {
            DetectiveTicket::Taxi => Self::Taxi,
            DetectiveTicket::Bus => Self::Bus,
            DetectiveTicket::Underground => Self::Underground,
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
pub enum DetectiveTicket {
    Taxi,
    Bus,
    Underground,
}

impl DetectiveTicket {
    pub const ALL: [Self; 3] = [Self::Taxi, Self::Bus, Self::Underground];
}

impl Display for DetectiveTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectiveTicket::Taxi => f.write_str("a taxi ticket"),
            DetectiveTicket::Bus => f.write_str("a bug ticket"),
            DetectiveTicket::Underground => f.write_str("an underground ticket"),
        }
    }
}
