use crate::magic::{BISHOP_DELTA, Magic, ROOK_DELTA, bishop_magic_table, rook_magic_table, slider_attacks};

const FILE_A: u64 = 0x0101010101010101;
const FILE_H: u64 = 0x8080808080808080;
const FILE_AB: u64 = 0x0303030303030303;
const FILE_GH: u64 = 0xc0c0c0c0c0c0c0c0;

pub const BISHOP_TABLE_SIZE: usize = 5248;
pub const ROOK_TABLE_SIZE: usize = 102400;

pub fn pawn_table() -> [[u64; 64]; 2] {
    let mut table = [[0; 64]; 2];

    for square in 0..64 {
        let bitboard = 1u64 << square;

        table[0][square] = (bitboard & !FILE_A) << 7 | (bitboard & !FILE_H) << 9;
        table[1][square] = (bitboard & !FILE_H) >> 7 | (bitboard & !FILE_A) >> 9;
    }

    table
}

pub fn king_table() -> Vec<u64> {
    let mut table = vec![0; 64];

    for square in 0..64 {
        let bitboard = 1u64 << square;

        table[square] =
            bitboard << 8 |
            bitboard >> 8 |
            (bitboard >> 1 | bitboard >> 9 | bitboard << 7) & !FILE_H |
            (bitboard << 1 | bitboard << 9 | bitboard >> 7) & !FILE_A;
    }

    table
}

pub fn knight_table() -> Vec<u64> {
    let mut table = vec![0; 64];

    for square in 0..64 {
        let bitboard = 1u64 << square;

        table[square] =
            (bitboard << 6 | bitboard >> 10) & !FILE_GH |
            (bitboard << 10 | bitboard >> 6) & !FILE_AB |
            (bitboard << 17 | bitboard >> 15) & !FILE_A |
            (bitboard << 15 | bitboard >> 17) & !FILE_H;
    }

    table
}

pub fn slider_table<const SIZE: usize>(magics: &Vec<Magic>, delta: [(i32, i32); 4]) -> Vec<u64> {
    let mut table = vec![0; SIZE];

    for square in 0..64 {
        let mut occupied = 0;

        loop {
            table[magics[square].index(occupied)] = slider_attacks(square, occupied, delta);

            occupied = occupied.wrapping_sub(magics[square].mask) & magics[square].mask;

            if occupied == 0 {
                break;
            }
        }
    }

    table
}

pub fn bishop_table() -> Vec<u64> {
    slider_table::<BISHOP_TABLE_SIZE>(&bishop_magic_table(), BISHOP_DELTA)
}

pub fn rook_table() -> Vec<u64> {
    slider_table::<ROOK_TABLE_SIZE>(&rook_magic_table(), ROOK_DELTA)
}