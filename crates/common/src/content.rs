use std::range::Range;

pub const TAXI_TICKET_COUNT: u8 = 57;
pub const BUS_TICKET_COUNT: u8 = 45;
pub const UNDERGROUND_TICKET_COUNT: u8 = 23;
pub const BLACK_TICKET_COUNT: u8 = 5;
pub const DOUBLE_MOVE_TICKET_COUNT: u8 = 2;

pub const DETECTIVE_INITIAL_UNDERGROUND_TICKET_COUNT: u8 = 4;
pub const DETECTIVE_INITIAL_TAXI_TICKET_COUNT: u8 = 11;
pub const DETECTIVE_INITIAL_BUS_TICKET_COUNT: u8 = 8;

pub const MIN_DETECTIVES: u8 = 1;
pub const MAX_DETECTIVES: u8 = 4;

pub const DETECTIVE_COUNT_RANGE: Range<u8> = Range {
    start: MIN_DETECTIVES,
    end: MAX_DETECTIVES + 1,
};

pub const ROUNDS: u8 = 22;
