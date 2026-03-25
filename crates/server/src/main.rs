use scotland_yard_common::{Field, destinations};

mod detectives;
mod mrx;
mod state;

fn main() {
    for x in destinations(Field::new(2).unwrap()) {
        println!("{x:?}")
    }
}
