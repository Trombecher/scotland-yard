use std::{fmt::Display, num::NonZeroU8};

/// A station.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Station(NonZeroU8);

impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "station {}", self.0)
    }
}

impl Station {
    const MAX_ID: NonZeroU8 = NonZeroU8::new(199).unwrap();

    #[must_use]
    pub const fn new(id: u8) -> Option<Self> {
        if let Some(id) = NonZeroU8::new(id)
            && id.get() <= Self::MAX_ID.get()
        {
            Some(Self(id))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        self.0.get()
    }
}

pub static STARTING_FIELDS: [Station; 16] = [
    Station::new(13).unwrap(),
    Station::new(26).unwrap(),
    Station::new(29).unwrap(),
    Station::new(34).unwrap(),
    Station::new(50).unwrap(),
    Station::new(53).unwrap(),
    Station::new(91).unwrap(),
    Station::new(94).unwrap(),
    Station::new(112).unwrap(),
    Station::new(117).unwrap(),
    Station::new(132).unwrap(),
    Station::new(138).unwrap(),
    Station::new(141).unwrap(),
    Station::new(155).unwrap(),
    Station::new(174).unwrap(),
    Station::new(197).unwrap(),
];
