use crate::board::{ConnectionType, Field};
use crate::controller::{MrXController, SeekerController};
use crate::game::{AdditionalSeekerInfo, MrXMove, MrXOpaqueMove, MrXTicket, Ticket};
use rand::{Rng, rng};

#[derive(Debug, Clone)]
pub struct PrimitiveSeeker {
    field: Option<Field>,
    taxi_tickets: u8,
    bus_tickets: u8,
    metro_tickets: u8,
}

impl PrimitiveSeeker {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            field: None,
            taxi_tickets: 10,
            bus_tickets: 8,
            metro_tickets: 4,
        }
    }
}

impl SeekerController for PrimitiveSeeker {
    fn init(&mut self, starting_field: Field) {
        self.field = Some(starting_field);
    }

    fn next_move(
        &mut self,
        _last_revealed_mr_x_position: Field,
        _last_mr_x_move: MrXOpaqueMove,
    ) -> Option<(Ticket, Field)> {
        let options: Vec<_> = self
            .field
            .unwrap()
            .connections()
            .iter()
            .copied()
            .filter_map(|(conn, field)| {
                if conn == ConnectionType::Taxi && self.taxi_tickets == 0 {
                    return None;
                }

                if conn == ConnectionType::Bus && self.bus_tickets == 0 {
                    return None;
                }

                if conn == ConnectionType::Metro && self.metro_tickets == 0 {
                    return None;
                }

                Ticket::try_from(conn).ok().map(|ticket| (ticket, field))
            })
            .collect();

        if options.is_empty() {
            return None;
        }

        let selected = options[rng().random_range(0..options.len())];
        self.field = Some(selected.1);

        match selected.0 {
            Ticket::Taxi => self.taxi_tickets -= 1,
            Ticket::Bus => self.bus_tickets -= 1,
            Ticket::Metro => self.metro_tickets -= 1,
        }

        Some(selected)
    }
}

pub struct PrimitiveMrX {
    field: Option<Field>,
}

impl PrimitiveMrX {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { field: None }
    }
}

impl MrXController for PrimitiveMrX {
    fn init(&mut self, starting_field: Field) {
        self.field = Some(starting_field);
    }

    fn next_move(
        &mut self,
        _seeker_positions: &[Field],
        _additional_seeker_info: Option<&[AdditionalSeekerInfo]>,
    ) -> MrXMove {
        let connections = self.field.unwrap().connections();
        
        let mov = connections[rng().random_range(0..connections.len())];
        
        MrXMove::Single(match mov.0 {
            ConnectionType::Taxi => MrXTicket::Taxi,
            ConnectionType::Bus => MrXTicket::Bus,
            ConnectionType::Metro => MrXTicket::Metro,
            ConnectionType::Ship => MrXTicket::Black,
        }, mov.1)
    }
}
