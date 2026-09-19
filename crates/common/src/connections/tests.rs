use crate::CONNECTIONS;

#[test]
fn no_directed_edges() {
    assert_eq!(CONNECTIONS.directed_edges().next(), None);
}

#[test]
fn inclusion() {
    assert!(CONNECTIONS.iter().all(|c| CONNECTIONS.has(c)));
}
