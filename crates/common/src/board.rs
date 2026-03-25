use std::mem::transmute;
use std::num::NonZeroU8;
use crate::connections::{OPTIONS_TABLE, TYPES_TABLE};
use crate::game::Ticket;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Field(NonZeroU8);

pub static STARTING_FIELDS: [Field; 16] = unsafe {
    transmute::<[u8; 16], _>([
        13, 26, 29, 34, 50, 53, 91, 94, 112, 117, 132, 138, 141, 155, 174, 197,
    ])
};

impl Field {
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(field: u8) -> Self {
        #[cfg(debug_assertions)]
        if field == 0 || field >= 200 {
            panic!(":(")
        }
        
        unsafe { Self(NonZeroU8::new_unchecked(field)) }
    }
    
    #[inline]
    pub fn connections(self) -> &'static [(ConnectionType, Field)] {
        OPTIONS_TABLE[self.0.get() as usize - 1]
    }
    
    #[inline]
    pub fn connection_with(self, other: Field) -> Option<ConnectionType> {
        TYPES_TABLE[self.0.get() as usize - 1][other.0.get() as usize - 1]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ConnectionType {
    Taxi,
    Bus,
    Metro,
    Ship
}

impl ConnectionType {
    #[inline]
    pub fn usable_with_ticket(self, ticket: Ticket) -> bool {
        match self {
            Self::Taxi if ticket == Ticket::Taxi => true,
            Self::Bus if ticket == Ticket::Bus => true,
            Self::Metro if ticket == Ticket::Metro => true,
            _ => false,
        }
    }
}