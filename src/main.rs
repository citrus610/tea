#![allow(dead_code)]

use crate::chess::{bitboard::Bitboard, board::Board, movegen::perft, moves::{Move, MoveKind}, square::Square};

mod chess;

fn main() {
    println!("Hello, world!");

    // for a in 0..64 {
    //     for b in 0..64 {
    //         let m = Bitboard::from_line(Square::from_raw(a), Square::from_raw(b)) | Bitboard::from_raw(1 << a) | Bitboard::from_raw(1 << b);

    //         println!("{}", m);

    //         let mut input = String::new();

    //         std::io::stdin().read_line(&mut input).expect("w");
    //     }
    // }

    let mut board = Board::startpos().unwrap();

    // board.make(Move::new(Square::A2, Square::A4, MoveKind::Normal));
    // board.make(Move::new(Square::A7, Square::A6, MoveKind::Normal));
    // board.make(Move::new(Square::A4, Square::A5, MoveKind::Normal));
    // board.make(Move::new(Square::B7, Square::B5, MoveKind::Normal));

    println!("{}", perft::<true>(&mut board, 7));
}