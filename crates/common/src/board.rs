use std::num::NonZeroU8;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Field(NonZeroU8);

pub static STARTING_FIELDS: [Field; 16] = [
    Field::new(13).unwrap(),
    Field::new(26).unwrap(),
    Field::new(29).unwrap(),
    Field::new(34).unwrap(),
    Field::new(50).unwrap(),
    Field::new(53).unwrap(),
    Field::new(91).unwrap(),
    Field::new(94).unwrap(),
    Field::new(112).unwrap(),
    Field::new(117).unwrap(),
    Field::new(132).unwrap(),
    Field::new(138).unwrap(),
    Field::new(141).unwrap(),
    Field::new(155).unwrap(),
    Field::new(174).unwrap(),
    Field::new(197).unwrap(),
];

impl Field {
    #[must_use]
    pub const fn new(index: u8) -> Option<Self> {
        if 0 < index && index < 200 {
            Some(Self(NonZeroU8::new(index).unwrap()))
        } else {
            None
        }
    }
}
