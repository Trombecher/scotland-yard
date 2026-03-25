use crate::board::{ConnectionType, Field, STARTING_FIELDS};
use crate::controller::{MrXController, SeekerController};
use crate::pile::Pile;
use rand::{Rng, rng};
use std::iter::from_fn;

const MAX_PLAYERS: u8 = 15;

impl TryFrom<MrXTicket> for Ticket {
    type Error = ();

    fn try_from(value: MrXTicket) -> Result<Self, Self::Error> {
        match value {
            MrXTicket::Taxi => Ok(Self::Taxi),
            MrXTicket::Bus => Ok(Self::Bus),
            MrXTicket::Metro => Ok(Self::Metro),
            MrXTicket::Black => Err(()),
        }
    }
}

impl TryFrom<ConnectionType> for Ticket {
    type Error = ();

    fn try_from(value: ConnectionType) -> Result<Self, Self::Error> {
        match value {
            ConnectionType::Taxi => Ok(Self::Taxi),
            ConnectionType::Bus => Ok(Self::Bus),
            ConnectionType::Metro => Ok(Self::Metro),
            ConnectionType::Ship => Err(()),
        }
    }
}

pub struct Game<'a> {
    seekers: Vec<&'a mut dyn SeekerController>,
    mr_x: &'a mut dyn MrXController,
    mr_x_starting_position: Field,
    mr_x_taxi_tickets: u8,
    mr_x_bus_tickets: u8,
    mr_x_metro_tickets: u8,
    mr_x_black_tickets: u8,
    mr_x_double_tickets: u8,
    mr_x_history: Vec<MrXMove>,
    mr_x_target: Field,
    seeker_info: Vec<SeekerInfo>,
    state: GameState,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SeekerInfo {
    starting_position: Field,
    taxi_tickets: u8,
    bus_tickets: u8,
    metro_tickets: u8,
    history: Vec<(Ticket, Field)>,
}

impl SeekerInfo {
    pub fn new(starting_positions: &mut Pile<Field>) -> Self {
        Self {
            starting_position: starting_positions.pick(),
            taxi_tickets: 10,
            bus_tickets: 8,
            metro_tickets: 4,
            history: vec![],
        }
    }

    pub fn field(&self) -> Field {
        self.history
            .last()
            .map(|(_, field)| field)
            .copied()
            .unwrap_or(self.starting_position)
    }

    /// Removes a ticket. Panics the player cannot use that ticket.
    pub fn use_ticket(&mut self, ticket: Ticket) {
        match ticket {
            Ticket::Taxi => {
                self.taxi_tickets = self
                    .taxi_tickets
                    .checked_sub(1)
                    .expect("No more taxi tickets")
            }
            Ticket::Bus => {
                self.bus_tickets = self
                    .bus_tickets
                    .checked_sub(1)
                    .expect("No more bus tickets")
            }
            Ticket::Metro => {
                self.metro_tickets = self
                    .metro_tickets
                    .checked_sub(1)
                    .expect("No more metro tickets")
            }
        }
    }

    /// Tells weather this [Seeker] can use this [ConnectionType] by spending a ticket.
    pub fn can_use(&self, ct: ConnectionType) -> bool {
        match ct {
            ConnectionType::Taxi if self.taxi_tickets > 0 => true,
            ConnectionType::Bus if self.bus_tickets > 0 => true,
            ConnectionType::Metro if self.metro_tickets > 0 => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    SeekerTurn(u8),
    MrXTurn,
    SeekerWon(u8),
    MrXWon,
}

impl<'a> Game<'a> {
    pub fn new(
        seekers: impl Iterator<Item = &'a mut dyn SeekerController>,
        mr_x: &'a mut dyn MrXController,
    ) -> Self {
        let seekers: Vec<&'a mut dyn SeekerController> = seekers.collect();
        let seeker_count = seekers.len() as u8;

        if seeker_count >= MAX_PLAYERS {
            panic!("Max player count exceeded");
        }

        if seeker_count == 0 {
            panic!("No seekers given");
        }

        let mut starting_positions = Pile::from(&STARTING_FIELDS);
        let mr_x_starting_position = starting_positions.pick();

        let mr_x_target = STARTING_FIELDS[rng().random_range(0..STARTING_FIELDS.len())];

        Self {
            seekers,
            mr_x,
            mr_x_starting_position,
            mr_x_taxi_tickets: 4,
            mr_x_bus_tickets: 4,
            mr_x_metro_tickets: 4,
            mr_x_black_tickets: seeker_count,
            mr_x_double_tickets: 2,
            mr_x_history: vec![],
            mr_x_target,
            seeker_info: (0..seeker_count)
                .map(|_| SeekerInfo::new(&mut starting_positions))
                .collect(),
            state: GameState::SeekerTurn(0),
        }
    }

    /// Returns the field Mr. X is on.
    pub fn mr_x_field(&self) -> Field {
        self.mr_x_history
            .last()
            .map(|mv| match mv {
                MrXMove::Single(_, mv) => mv,
                MrXMove::Double(_, (_, mv)) => mv,
            })
            .copied()
            .unwrap_or(self.mr_x_starting_position)
    }

    pub fn step(&mut self) {
        self.state = match self.state {
            GameState::SeekerTurn(seeker_id) => {
                let seeker = self.seekers[seeker_id as usize];
                let mv = seeker.next_move(
                    self.mr_x_starting_position, // TODO: fix this
                    self.mr_x_history.last().copied().unwrap().into(),
                );

                let seeker_info = &mut self.seeker_info[seeker_id as usize];

                if let Some((ticket, field)) = mv {
                    if !seeker_info
                        .field()
                        .connection_with(field)
                        .is_some_and(|ct| ct.usable_with_ticket(ticket))
                    {
                        panic!("invalid move")
                    }

                    seeker_info.use_ticket(ticket);

                    seeker_info.history.push((ticket, field));
                } else if seeker_info
                    .field()
                    .connections()
                    .iter()
                    .any(|(ct, _)| seeker_info.can_use(*ct))
                {
                    panic!("Can use ticket!")
                }

                if seeker_id == (self.seekers.len() - 1) as u8 {
                    // Seeker is last

                    // TODO: some stuff maybe

                    GameState::MrXTurn
                } else {
                    // Next seeker's turn
                    GameState::SeekerTurn(seeker_id + 1)
                }
            }
            GameState::MrXTurn => {
                let seeker_positions: Vec<_> =
                    self.seeker_info.iter().map(SeekerInfo::field).collect();

                let mv = self.mr_x.next_move(&seeker_positions, None);
            }
            GameState::SeekerWon(x) => GameState::SeekerWon(x),
            GameState::MrXWon => GameState::MrXWon,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MrXMove {
    Single(MrXTicket, Field),
    Double((MrXTicket, Field), (MrXTicket, Field)),
}

impl MrXMove {
    pub fn iter(self) -> impl Iterator<Item = (MrXTicket, Field)> {
        let mut mv = Some(self);

        from_fn(move || match mv {
            Some(Self::Single(a, b)) => {
                mv = None;
                Some((a, b))
            }
            Some(Self::Double(a, b)) => {
                mv = Some(Self::Single(b.0, b.1));
                Some(a)
            }
            None => None,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MrXOpaqueMove {
    Single(MrXTicket),
    Double(MrXTicket, MrXTicket),
}

impl From<MrXMove> for MrXOpaqueMove {
    fn from(value: MrXMove) -> Self {
        match value {
            MrXMove::Single(x, _) => Self::Single(x),
            MrXMove::Double((x, _), (y, _)) => Self::Double(x, y),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdditionalSeekerInfo {
    taxi_tickets: u8,
    bus_tickets: u8,
    metro_tickets: u8,
}
