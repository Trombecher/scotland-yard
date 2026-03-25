mod detectives;
mod mrx;
mod state;

use scotland_yard_common::CONNECTIONS;

fn main() {
    println!("{}", CONNECTIONS.is_sorted());
}
