mod data;
#[cfg(test)]
mod tests;

use std::{fmt::Display, iter::Copied, slice};

use crate::{DetectiveTicket, Station};

pub use data::CONNECTIONS;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Connection {
    pub from: Station,
    pub kind: ConnectionKind,
    pub to: Station,
}

impl Connection {
    #[must_use]
    pub fn reverse(self) -> Self {
        Self {
            from: self.to,
            kind: self.kind,
            to: self.from,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ConnectionKind {
    Taxi,
    Bus,
    Underground,
    Black,
}

impl Display for ConnectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Taxi => f.write_str("taxi"),
            Self::Bus => f.write_str("bus"),
            Self::Underground => f.write_str("underground"),
            Self::Black => f.write_str("black"),
        }
    }
}

impl ConnectionKind {
    #[inline]
    #[must_use]
    pub fn is_usable_with_ticket(self, ticket: DetectiveTicket) -> bool {
        match self {
            Self::Taxi if ticket == DetectiveTicket::Taxi => true,
            Self::Bus if ticket == DetectiveTicket::Bus => true,
            Self::Underground if ticket == DetectiveTicket::Underground => true,
            _ => false,
        }
    }
}

pub struct ConnectionGraph<const N: usize> {
    pub connections: [Connection; N],
}

impl<const N: usize> ConnectionGraph<N> {
    #[must_use]
    pub fn has(&self, connection: Connection) -> bool {
        self.connections.binary_search(&connection).is_ok()
    }

    pub fn directed_edges(&self) -> impl Iterator<Item = Connection> {
        self.iter().filter(|edge| !self.has(edge.reverse()))
    }

    pub fn iter(&self) -> Copied<slice::Iter<'_, Connection>> {
        self.connections.iter().copied()
    }

    pub fn destinations(&self, from: Station) -> impl Iterator<Item = (ConnectionKind, Station)> {
        let start = self
            .connections
            .partition_point(|Connection { from: station, .. }| *station < from);

        self.connections[start..]
            .iter()
            .copied()
            .take_while(move |Connection { from: station, .. }| *station == from)
            .map(|Connection { kind, to, .. }| (kind, to))
    }
}

impl<'a, const N: usize> IntoIterator for &'a ConnectionGraph<N> {
    type Item = Connection;

    type IntoIter = Copied<slice::Iter<'a, Connection>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
