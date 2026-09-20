use super::*;

#[test]
fn connections_are_sorted() {
    assert!(CONNECTIONS.connections.is_sorted());
}

#[test]
fn no_directed_edges() {
    assert_eq!(CONNECTIONS.directed_edges().next(), None);
}

#[test]
fn inclusion() {
    assert!(CONNECTIONS.iter().all(|c| CONNECTIONS.has(c)));
}

#[test]
fn destinations_of_station_67() {
    const STATION_67: Station = Station::new(67).unwrap();

    let mut destinations_of_67_and_taxi =
        CONNECTIONS.destinations(STATION_67, ConnectionKind::Taxi);

    assert_eq!(destinations_of_67_and_taxi.next(), Station::new(51));
    assert_eq!(destinations_of_67_and_taxi.next(), Station::new(66));
    assert_eq!(destinations_of_67_and_taxi.next(), Station::new(68));
    assert_eq!(destinations_of_67_and_taxi.next(), Station::new(84));
    assert_eq!(destinations_of_67_and_taxi.next(), None);

    let mut destinations_of_67_and_bus = CONNECTIONS.destinations(STATION_67, ConnectionKind::Bus);

    assert_eq!(destinations_of_67_and_bus.next(), Station::new(23));
    assert_eq!(destinations_of_67_and_bus.next(), Station::new(52));
    assert_eq!(destinations_of_67_and_bus.next(), Station::new(65));
    assert_eq!(destinations_of_67_and_bus.next(), Station::new(82));
    assert_eq!(destinations_of_67_and_bus.next(), Station::new(102));
    assert_eq!(destinations_of_67_and_bus.next(), None);

    let mut destinations_of_67_and_underground =
        CONNECTIONS.destinations(STATION_67, ConnectionKind::Underground);

    assert_eq!(destinations_of_67_and_underground.next(), Station::new(13));
    assert_eq!(destinations_of_67_and_underground.next(), Station::new(79));
    assert_eq!(destinations_of_67_and_underground.next(), Station::new(89));
    assert_eq!(destinations_of_67_and_underground.next(), Station::new(111));
    assert_eq!(destinations_of_67_and_underground.next(), None);

    let mut destinations_of_67_and_black =
        CONNECTIONS.destinations(STATION_67, ConnectionKind::Black);

    assert_eq!(destinations_of_67_and_black.next(), None);
}
