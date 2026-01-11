#![allow(dead_code)]

use crate::chess::{attack::*, bitboard::Bitboard, color::Color, file::File, rank::Rank, square::Square};

mod chess;

fn main() {
    println!("Hello, world!");

    let occupied = Bitboard::from_file(File::C) | Bitboard::from_file(File::G) | Bitboard::from_rank(Rank::Third) | Bitboard::from_rank(Rank::Seventh);

    println!("{}", pawn_attacks(Square::D4, Color::White));
    println!("{}", king_attacks(Square::D4));
    println!("{}", knight_attacks(Square::D4));
    println!("{}", occupied);
    println!("{}", bishop_attacks(Square::D4, occupied) | occupied);
    println!("{}", rook_attacks(Square::D4, occupied) | occupied);
}