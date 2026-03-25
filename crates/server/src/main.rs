use scotland_yard_common::{CONNECTIONS, Field, destinations, is_connection};

mod detectives;
mod mrx;
mod state;

fn main() {
    println!("{}", CONNECTIONS.is_sorted())
}
