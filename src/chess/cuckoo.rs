use crate::chess::{bitboard::Bitboard, moves::{Move, MoveKind}, piece::{Piece, PieceKind}, square::Square, zobrist::{ZOBRIST, Zobrist}};

pub struct Cuckoo {
    pub hash: [u64; 8192],
    pub mv: [Move; 8192]
}

const fn hash_1(hash: u64) -> u64 {
    hash & 0x1FFFu64
}

const fn hash_2(hash: u64) -> u64 {
    (hash >> 16) & 0x1FFFu64
}

const fn is_reversible(kind: PieceKind, from: Square, to: Square) -> bool {
    debug_assert!(kind.value() != PieceKind::Pawn.value());

    let mut attack = Bitboard::new();

    // Blah

    attack.value() & Bitboard::from_square(to).value() != 0
}

#[allow(long_running_const_eval)]
pub const CUCKOO: Cuckoo = {
    let mut cuckoo = Cuckoo {
        hash: [0; 8192],
        mv: [Move::NULL; 8192],
    };

    let mut piece = Piece::WhiteKnight.value();

    while piece < Piece::COUNT as u8 {
        let mut a = 0;

        while a < 64 {
            let mut b = a + 1;

            while b < 64 {
                let kind = Piece::from_raw(piece).kind();
                let from = Square::from_raw(a);
                let to = Square::from_raw(b);

                if !is_reversible(kind, from, to) {
                    continue;
                }

                let mut mv = Move::new(from, to, MoveKind::Normal);
                let mut hash = ZOBRIST.piece[piece as usize][a as usize] ^ ZOBRIST.piece[piece as usize][b as usize] ^ ZOBRIST.color;
                let mut index = hash_1(hash);

                loop {
                    std::mem::swap(&mut cuckoo.hash[index as usize], &mut hash);
                    std::mem::swap(&mut cuckoo.mv[index as usize], &mut mv);

                    if mv.is_null() {
                        break;
                    }

                    if index == hash_1(hash) {
                        index = hash_2(hash);
                    }
                    else {
                        index = hash_1(hash);
                    }
                }

                b += 1;
            }

            a += 1;
        }

        piece += 1;
    }

    cuckoo
};