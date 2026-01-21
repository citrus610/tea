#![allow(dead_code)]
use std::time::Instant;

use crate::chess::{board::Board, movegen::perft};

mod chess;
mod engine;

fn main() {
    let mut board = Board::startpos().unwrap();

    let time = Instant::now();
    let count = perft::<true>(&mut board, 7);
    let elasped = time.elapsed().as_millis();
    let knps = count as u128 / elasped;

    println!("perft: {}", count);
    println!("time: {} ms", elasped);
    println!("nps: {} kn/s", knps);
}