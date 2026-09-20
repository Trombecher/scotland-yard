use scotland_yard_common::{DetectiveTicket, MrXTicket, content};

use crate::mr_x::errors::{MrXMoveError, MrXNoTicketError};

#[derive(Copy, Clone, Debug)]
pub struct RemainingMrXTickets {
    double_move: u8,
    bus: u8,
    taxi: u8,
    underground: u8,
    black: u8,
}

impl RemainingMrXTickets {
    #[must_use]
    pub const fn new(detective_count: u8) -> Self {
        Self {
            double_move: content::DOUBLE_MOVE_TICKET_COUNT,
            bus: content::mr_x_initial_bus_ticket_count(detective_count),
            taxi: content::mr_x_initial_taxi_ticket_count(detective_count),
            underground: content::mr_x_initial_underground_ticket_count(detective_count),
            black: content::BLACK_TICKET_COUNT,
        }
    }

    #[must_use]
    pub const fn add_detective_ticket(self, ticket: DetectiveTicket) -> Self {
        macro_rules! add_ticket {
            ($field:ident, $name:literal) => {
                Self {
                    $field: self.$field.checked_add(1).expect(concat!(
                        "too many ",
                        $name,
                        " tickets"
                    )),
                    ..self
                }
            };
        }

        match ticket {
            DetectiveTicket::Taxi => add_ticket!(taxi, "taxi"),
            DetectiveTicket::Bus => add_ticket!(bus, "bus"),
            DetectiveTicket::Underground => {
                add_ticket!(underground, "underground")
            }
        }
    }

    #[must_use]
    pub const fn remaining_double_move_tickets(self) -> u8 {
        self.double_move
    }

    #[must_use]
    pub const fn remaining_bus_tickets(self) -> u8 {
        self.bus
    }

    #[must_use]
    pub const fn remaining_taxi_tickets(self) -> u8 {
        self.taxi
    }

    #[must_use]
    pub const fn remaining_underground_tickets(self) -> u8 {
        self.underground
    }

    /// # Errors
    ///
    /// TODO
    pub const fn use_ticket(self, ticket: MrXTicket) -> Result<Self, MrXNoTicketError> {
        macro_rules! decrement_remaining {
            ($field:ident) => {{
                if let Some(decremented) = self.$field.checked_sub(1) {
                    Ok(Self {
                        $field: decremented,
                        ..self
                    })
                } else {
                    Err(MrXNoTicketError(ticket))
                }
            }};
        }

        match ticket {
            MrXTicket::Taxi => decrement_remaining!(taxi),
            MrXTicket::Bus => decrement_remaining!(bus),
            MrXTicket::Underground => decrement_remaining!(underground),
            MrXTicket::Black => decrement_remaining!(black),
        }
    }

    /// # Errors
    ///
    /// TODO
    pub const fn use_double_move_ticket(self) -> Result<Self, MrXMoveError> {
        if let Some(new_remaining_double_move_tickets) = self.double_move.checked_sub(1) {
            Ok(Self {
                double_move: new_remaining_double_move_tickets,
                ..self
            })
        } else {
            Err(MrXMoveError::NoDoubleMoveTicketLeft)
        }
    }

    #[must_use]
    pub fn has_at_least_one_ticket_remaining_of(self, ticket: MrXTicket) -> bool {
        match ticket {
            MrXTicket::Taxi => self.taxi > 0,
            MrXTicket::Bus => self.bus > 0,
            MrXTicket::Underground => self.underground > 0,
            MrXTicket::Black => self.black > 0,
        }
    }

    /// Emits all ticket (kinds) which there is at
    /// least one remaining of.
    pub fn available_tickets(self) -> impl Iterator<Item = MrXTicket> {
        MrXTicket::ALL
            .into_iter()
            .filter(move |ticket| self.has_at_least_one_ticket_remaining_of(*ticket))
    }
}
