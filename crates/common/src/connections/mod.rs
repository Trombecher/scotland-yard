mod data;
#[cfg(test)]
mod tests;

use std::{iter::Copied, slice};

use crate::Ticket;

pub use data::CONNECTIONS;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Station(pub u8);

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

impl ConnectionKind {
    #[inline]
    pub fn is_usable_with_ticket(self, ticket: Ticket) -> bool {
        match self {
            Self::Taxi if ticket == Ticket::Taxi => true,
            Self::Bus if ticket == Ticket::Bus => true,
            Self::Underground if ticket == Ticket::Underground => true,
            _ => false,
        }
    }
}

pub struct ConnectionGraph<const N: usize> {
    pub connections: [Connection; N],
}

impl<const N: usize> ConnectionGraph<N> {
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
