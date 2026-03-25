use crate::controller::{PrimitiveMrX, PrimitiveSeeker};
use crate::game::Game;

mod board;
mod connections;
mod controller;
mod game;
mod pile;

fn main() {
    let mut mr_x = PrimitiveMrX::new();
    let mut seekers = [const { PrimitiveSeeker::new() }; 5];

    let game = Game::new(seekers.iter_mut().map(|r| r as _), &mut mr_x);

    println!("Hello, world!");
}
