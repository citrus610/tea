include!(concat!(env!("OUT_DIR"), "/attack.rs"));

use crate::chess::{bitboard::Bitboard, color::Color, direction::Direction, square::Square, magic::{BISHOP_MAGICS, ROOK_MAGICS}};

#[inline(always)]
pub const fn pawn_east_attacks(square: Square, color: Color) -> Bitboard {
    match color {
        Color::White => Bitboard::from_square(square).shift(Direction::NorthEast),
        Color::Black => Bitboard::from_square(square).shift(Direction::SouthWest)
    }
}

#[inline(always)]
pub const fn pawn_west_attacks(square: Square, color: Color) -> Bitboard {
    match color {
        Color::White => Bitboard::from_square(square).shift(Direction::NorthWest),
        Color::Black => Bitboard::from_square(square).shift(Direction::SouthEast)
    }
}

#[inline(always)]
pub const fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    Bitboard::from_raw(PAWN_ATTACKS[color.index()][square.index()])
}

#[inline(always)]
pub const fn king_attacks(square: Square) -> Bitboard {
    Bitboard::from_raw(KING_ATTACKS[square.index()])
}

#[inline(always)]
pub const fn knight_attacks(square: Square) -> Bitboard {
    Bitboard::from_raw(KNIGHT_ATTACKS[square.index()])
}

#[inline(always)]
pub const fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    Bitboard::from_raw(BISHOP_ATTACKS[BISHOP_MAGICS[square.index()].index(occupied)])
}

#[inline(always)]
pub const fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    Bitboard::from_raw(ROOK_ATTACKS[ROOK_MAGICS[square.index()].index(occupied)])
}