use scotland_yard_common::connections::{CONNECTIONS, Station};

fn main() {
    println!(
        "{:#?}",
        CONNECTIONS.destinations(Station(67)).collect::<Vec<_>>()
    )
}
